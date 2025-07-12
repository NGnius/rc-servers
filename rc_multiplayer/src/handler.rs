pub struct LnlEventHandler {
    event_handlers: std::collections::HashMap<i16, Box<dyn super::EventCodeHandler>>,
    motion_handler: Box<dyn super::RobotMotionHandler>,
    user_provider: std::sync::Arc<oj_rc_core::persist::user::UserImpl>
}

impl LnlEventHandler {
    pub fn new<M: super::RobotMotionHandler + 'static>(user_provider: std::sync::Arc<oj_rc_core::persist::user::UserImpl>, motion_handler: M) -> Self {
        Self {
            event_handlers: std::collections::HashMap::new(),
            motion_handler: Box::new(motion_handler),
            user_provider,
        }
    }

    pub fn add<H: super::EventCode + super::EventCodeHandler + 'static>(mut self, handler: H) -> Self {
        if self.event_handlers.insert(H::CODE, Box::new(handler)).is_some() {
            log::warn!("Replaced event handler {} with new handler", H::CODE);
        }
        self
    }
}

#[async_trait::async_trait]
impl literustlib_server::EventHandler for LnlEventHandler {
    type PacketData = super::PacketData;
    type UserData = super::UserData;

    async fn on_receive(&self, data: Self::PacketData, header: &literustlib::packet::Header, peer: &std::sync::Arc< literustlib_server::Connection<Self::PacketData>>, user: &Self::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<Self::PacketData>>) {
        log::debug!("Got message {:?} (len: {}) from connection id {}", data.message_ty, data.data.len(), peer.id());
        match data.message_ty {
            crate::data::MessageType::ClientMsg => {
                if let Some(handler) = self.event_handlers.get(&data.variant) {
                    handler.handle(&data.data, peer, user, sender).await;
                } else {
                    let variant_pretty = i16_to_event(data.variant).map(|x| format!("{:?}", x)).unwrap_or_else(|| "???".to_owned());
                    #[cfg(debug_assertions)]
                    {
                        panic!("Unsupported {:?} event variant {} ({}), pls fix!!!\n {:?}", header.property, variant_pretty, data.variant, &data.data[..]);
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        log::warn!("Unsupported {:?} event variant {} ({}); {:?}, pls fix!!!", header.property, variant_pretty, data.variant);
                    }
                }
            },
            crate::data::MessageType::ServerMsg => {
                log::debug!("Got message from server but I'm the server??? (ignoring)");
            },
            crate::data::MessageType::RobotMotion => {
                self.motion_handler.handle(&data.data, user).await;
                //log::warn!("Ignoring robot motion message");
            },
        }

    }

    async fn on_connect_start(&self, addr: &core::net::SocketAddr, key: String, peer: &std::sync::Arc< literustlib_server::Connection<Self::PacketData>>) -> Option<Self::UserData> {
        log::debug!("New connection started from {}!!! (key:{}, id:{})", addr, key, peer.id());
        //let mut buf = Vec::new();
        //literustlib::packet::Packet::with_data(literustlib::packet::Property::Reliable, &[9, 0, 0, 0, 0, 0]).dump(&mut buf).unwrap_or_default();
        //socket.send_to(&buf, addr).await.unwrap_or_default();
        Some(crate::UserData::new(self.user_provider.clone()))
    }

    async fn on_connect_done(&self, peer: &std::sync::Arc< literustlib_server::Connection<Self::PacketData>>, _user: &Self::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<Self::PacketData>>) {
        log::debug!("New connection completed (id:{})", peer.id());
        let data = EventData::without_data(
            crate::data::MessageType::ServerMsg,
            rlnl::event_code::NetworkEvent::OnConnectedToGameServer,
        );
        if let Err(e) = sender.send_data(data, literustlib::packet::Property::Reliable, peer).await {
            log::error!("Failed to send rlnl OnConnectedToGameServer event: {}", e);
        }

    }
}

#[derive(Debug)]
pub struct EventData {
    pub message_ty: crate::data::MessageType,
    pub variant: i16,
    pub data_size: u16,
    pub data: bytes::Bytes,
}

impl EventData {
    pub fn with_data(message_ty: crate::data::MessageType, event: rlnl::event_code::NetworkEvent, data: bytes::Bytes) -> Self {
        Self {
            message_ty,
            variant: event as i16,
            data_size: data.len().try_into().expect("Event data too large"),
            data,
        }
    }

    pub fn without_data(message_ty: crate::data::MessageType, event: rlnl::event_code::NetworkEvent) -> Self {
        Self {
            message_ty,
            variant: event as i16,
            data_size: 0,
            data: bytes::Bytes::new(),
        }
    }
}

impl literustlib::packet::PacketData for EventData {
    fn parse(bytes: bytes::Bytes, _header: &literustlib::packet::Header) -> std::io::Result<Self> {
        log::debug!("Got packet data ({}) {:?}", bytes.len(), &bytes[..]);
        if bytes.len() >= 6 {
            let data = bytes.slice(6..);
            let net_message_num = i16::from_le_bytes([bytes[0], bytes[1]]);
            let net_message_type = crate::data::MessageType::from_i16(net_message_num).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Unsupported, format!("Unsupported message type {}", net_message_num)))?;
            let variant = i16::from_le_bytes([bytes[2], bytes[3]]);
            let data_size = u16::from_le_bytes([bytes[4], bytes[5]]);
            Ok(Self {
                message_ty: net_message_type,
                variant,
                data_size,
                data,
            })
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Packet data is too short"))
        }

    }

    fn dump(&self) -> bytes::Bytes {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(&(self.message_ty as i16).to_le_bytes()).unwrap();
        buf.write_all(&(self.variant as i16).to_le_bytes()).unwrap();
        buf.write_all(&(self.data_size as u16).to_le_bytes()).unwrap();
        buf.write_all(&self.data).unwrap();
        buf.into()
    }
}

const fn i16_to_event(num: i16) -> Option<rlnl::event_code::NetworkEvent> {
    match num {
        0 => Some(rlnl::event_code::NetworkEvent::OnFailedToConnectToMasterServer),
        1 => Some(rlnl::event_code::NetworkEvent::OnConnectingToLobbyServer),
        2 => Some(rlnl::event_code::NetworkEvent::OnConnectedToLobbyServer),
        3 => Some(rlnl::event_code::NetworkEvent::OnDisconnectingFromLobbyServer),
        4 => Some(rlnl::event_code::NetworkEvent::OnDisconnectedFromLobbyServer),
        5 => Some(rlnl::event_code::NetworkEvent::OnConnectedToServer),
        6 => Some(rlnl::event_code::NetworkEvent::OnFailedToConnectToServer),
        7 => Some(rlnl::event_code::NetworkEvent::OnConnectionLost),
        8 => Some(rlnl::event_code::NetworkEvent::OnDisconnectedFromServer),
        9 => Some(rlnl::event_code::NetworkEvent::OnConnectedToGameServer),
        10 => Some(rlnl::event_code::NetworkEvent::OnServerStarted),
        11 => Some(rlnl::event_code::NetworkEvent::OnServerStopped),
        12 => Some(rlnl::event_code::NetworkEvent::OnPlayerConnectedToServer),
        13 => Some(rlnl::event_code::NetworkEvent::OnPlayerDisconnectedFromServer),
        14 => Some(rlnl::event_code::NetworkEvent::RequestRespawnPoint),
        15 => Some(rlnl::event_code::NetworkEvent::FreeSpawnPoint),
        16 => Some(rlnl::event_code::NetworkEvent::RequestTeamBaseModel),
        17 => Some(rlnl::event_code::NetworkEvent::RequestCapturePoints),
        18 => Some(rlnl::event_code::NetworkEvent::RequestEqualizerModel),
        19 => Some(rlnl::event_code::NetworkEvent::TeamBase),
        20 => Some(rlnl::event_code::NetworkEvent::RegisterCapturePoints),
        21 => Some(rlnl::event_code::NetworkEvent::RegisterEqualizer),
        22 => Some(rlnl::event_code::NetworkEvent::FreeRespawnPoint),
        23 => Some(rlnl::event_code::NetworkEvent::PlayerIDs),
        24 => Some(rlnl::event_code::NetworkEvent::SyncMachineCubes),
        25 => Some(rlnl::event_code::NetworkEvent::MachineDestroyed),
        26 => Some(rlnl::event_code::NetworkEvent::GameStarted),
        27 => Some(rlnl::event_code::NetworkEvent::OnPlayerInputChanged),
        28 => Some(rlnl::event_code::NetworkEvent::OnServerReceivedInputChange),
        29 => Some(rlnl::event_code::NetworkEvent::ClientUnregistered),
        30 => Some(rlnl::event_code::NetworkEvent::OnAnotherClientDisconnected),
        31 => Some(rlnl::event_code::NetworkEvent::OnClientReconnected),
        32 => Some(rlnl::event_code::NetworkEvent::DamageCube),
        33 => Some(rlnl::event_code::NetworkEvent::FireWeaponEffect),
        35 => Some(rlnl::event_code::NetworkEvent::FireMiss),
        36 => Some(rlnl::event_code::NetworkEvent::MultipleFireMisses),
        37 => Some(rlnl::event_code::NetworkEvent::CurrentGameTime),
        38 => Some(rlnl::event_code::NetworkEvent::EndGame),
        39 => Some(rlnl::event_code::NetworkEvent::GameWon),
        40 => Some(rlnl::event_code::NetworkEvent::GameLost),
        41 => Some(rlnl::event_code::NetworkEvent::GameWonBaseDestroyed),
        42 => Some(rlnl::event_code::NetworkEvent::GameLostBaseDestroyed),
        43 => Some(rlnl::event_code::NetworkEvent::BuffTeamPlayers),
        44 => Some(rlnl::event_code::NetworkEvent::PlayerThreateningBase),
        45 => Some(rlnl::event_code::NetworkEvent::TimeToGameStart),
        46 => Some(rlnl::event_code::NetworkEvent::SetRespawnWaitingTime),
        47 => Some(rlnl::event_code::NetworkEvent::TeamBaseState),
        48 => Some(rlnl::event_code::NetworkEvent::TeamBaseCaptureStart),
        49 => Some(rlnl::event_code::NetworkEvent::TeamBaseCaptureReset),
        50 => Some(rlnl::event_code::NetworkEvent::TeamBaseCaptureStop),
        51 => Some(rlnl::event_code::NetworkEvent::TeamBaseSectionComplete),
        52 => Some(rlnl::event_code::NetworkEvent::TeamBaseFinalSectionComplete),
        53 => Some(rlnl::event_code::NetworkEvent::TeamBaseInitialise),
        54 => Some(rlnl::event_code::NetworkEvent::GetClientPings),
        55 => Some(rlnl::event_code::NetworkEvent::SetClientPing),
        56 => Some(rlnl::event_code::NetworkEvent::WarnPlayer),
        57 => Some(rlnl::event_code::NetworkEvent::EnemySpotted),
        58 => Some(rlnl::event_code::NetworkEvent::RemoteEnemySpotted),
        59 => Some(rlnl::event_code::NetworkEvent::AssistBonusRequest),
        60 => Some(rlnl::event_code::NetworkEvent::TeamBaseContested),
        61 => Some(rlnl::event_code::NetworkEvent::AcquireRemoteAI),
        63 => Some(rlnl::event_code::NetworkEvent::KillBonusRequest),
        64 => Some(rlnl::event_code::NetworkEvent::EACRegisterToken),
        65 => Some(rlnl::event_code::NetworkEvent::HeallingAssistBonusRequest),
        66 => Some(rlnl::event_code::NetworkEvent::ProtectTeamMateBonusRequest),
        67 => Some(rlnl::event_code::NetworkEvent::DefendTheBaseBonusRequest),
        68 => Some(rlnl::event_code::NetworkEvent::DestroyCubesBonusRequest),
        69 => Some(rlnl::event_code::NetworkEvent::DestroyHealCubesPointsAwarded),
        70 => Some(rlnl::event_code::NetworkEvent::ConfirmedKill),
        71 => Some(rlnl::event_code::NetworkEvent::BonusesFlushDone),
        72 => Some(rlnl::event_code::NetworkEvent::AlignmentRectifierStarted),
        73 => Some(rlnl::event_code::NetworkEvent::HealCubesBonusRequest),
        74 => Some(rlnl::event_code::NetworkEvent::SetShieldState),
        75 => Some(rlnl::event_code::NetworkEvent::TeamBaseLowHealth),
        76 => Some(rlnl::event_code::NetworkEvent::AwardTeamBaseProtoniumDestroyedRequest),
        77 => Some(rlnl::event_code::NetworkEvent::InitialiseGameStats),
        78 => Some(rlnl::event_code::NetworkEvent::UpdateGameStats),
        79 => Some(rlnl::event_code::NetworkEvent::MapPingEvent),
        80 => Some(rlnl::event_code::NetworkEvent::SurrenderRequest),
        81 => Some(rlnl::event_code::NetworkEvent::HealSelf),
        82 => Some(rlnl::event_code::NetworkEvent::HealSelfResponse),
        83 => Some(rlnl::event_code::NetworkEvent::SurrenderVoteStarted),
        84 => Some(rlnl::event_code::NetworkEvent::SurrenderVoteCast),
        85 => Some(rlnl::event_code::NetworkEvent::CurrentSurrenderVotes),
        86 => Some(rlnl::event_code::NetworkEvent::SurrenderAccepted),
        87 => Some(rlnl::event_code::NetworkEvent::SurrenderDeclined),
        88 => Some(rlnl::event_code::NetworkEvent::SetSurrenderTimes),
        89 => Some(rlnl::event_code::NetworkEvent::SetFinalGameScore),
        90 => Some(rlnl::event_code::NetworkEvent::PitLeaderBoardUpdate),
        91 => Some(rlnl::event_code::NetworkEvent::PitModeState),
        93 => Some(rlnl::event_code::NetworkEvent::ValidateGameGuid),
        94 => Some(rlnl::event_code::NetworkEvent::GameGuidValidated),
        95 => Some(rlnl::event_code::NetworkEvent::PlayerInsideBase),
        97 => Some(rlnl::event_code::NetworkEvent::ConfirmedAssist),
        98 => Some(rlnl::event_code::NetworkEvent::LockOnNotification),
        99 => Some(rlnl::event_code::NetworkEvent::LockOnNotificationBroadcast),
        100 => Some(rlnl::event_code::NetworkEvent::ShieldSpawned),
        101 => Some(rlnl::event_code::NetworkEvent::SpawnShield),
        102 => Some(rlnl::event_code::NetworkEvent::BroadcastOpenShield),
        103 => Some(rlnl::event_code::NetworkEvent::OpenShield),
        104 => Some(rlnl::event_code::NetworkEvent::BroadcastInvisible),
        105 => Some(rlnl::event_code::NetworkEvent::MakeInvisible),
        106 => Some(rlnl::event_code::NetworkEvent::BroadcastVisible),
        107 => Some(rlnl::event_code::NetworkEvent::MakeVisible),
        108 => Some(rlnl::event_code::NetworkEvent::BroadcastActivateTeleportEffect),
        109 => Some(rlnl::event_code::NetworkEvent::ActivateTeleportEffect),
        110 => Some(rlnl::event_code::NetworkEvent::BroadcastActivateReadyEffect),
        111 => Some(rlnl::event_code::NetworkEvent::ActivateReadyEffect),
        112 => Some(rlnl::event_code::NetworkEvent::BroadcastSpawnEmpLocator),
        113 => Some(rlnl::event_code::NetworkEvent::SpawnEmpLocator),
        114 => Some(rlnl::event_code::NetworkEvent::BroadcastSpawnEmpMachineEffect),
        115 => Some(rlnl::event_code::NetworkEvent::SpawnEmpMachineEffect),
        116 => Some(rlnl::event_code::NetworkEvent::WeaponSelect),
        117 => Some(rlnl::event_code::NetworkEvent::BroadcastWeaponSelect),
        118 => Some(rlnl::event_code::NetworkEvent::HostAIs),
        119 => Some(rlnl::event_code::NetworkEvent::HealAlly),
        120 => Some(rlnl::event_code::NetworkEvent::HealAllyResponse),
        121 => Some(rlnl::event_code::NetworkEvent::SelfDestructClassicMode),
        122 => Some(rlnl::event_code::NetworkEvent::GameModeSettings),
        123 => Some(rlnl::event_code::NetworkEvent::TeamDeathMatchState),
        124 => Some(rlnl::event_code::NetworkEvent::ClientDisconnecting),
        125 => Some(rlnl::event_code::NetworkEvent::SendBonus),
        126 => Some(rlnl::event_code::NetworkEvent::TestConnection),
        127 => Some(rlnl::event_code::NetworkEvent::MachineDestroyedConfirmed),
        128 => Some(rlnl::event_code::NetworkEvent::EnergyModuleActivated),
        129 => Some(rlnl::event_code::NetworkEvent::BroadcastLoadingProgress),
        130 => Some(rlnl::event_code::NetworkEvent::RequestLoadingProgressAllUsers),
        131 => Some(rlnl::event_code::NetworkEvent::LoadingComplete),
        132 => Some(rlnl::event_code::NetworkEvent::GameAborted),
        134 => Some(rlnl::event_code::NetworkEvent::SendDamagedByEnemyShield),
        135 => Some(rlnl::event_code::NetworkEvent::DamagedByEnemyShield),
        136 => Some(rlnl::event_code::NetworkEvent::EqualizerNotification),
        137 => Some(rlnl::event_code::NetworkEvent::CapturePointProgress),
        138 => Some(rlnl::event_code::NetworkEvent::CapturePointNotification),
        140 => Some(rlnl::event_code::NetworkEvent::RadarModuleActivated),
        141 => Some(rlnl::event_code::NetworkEvent::RemoteRadarModuleActivated),
        142 => Some(rlnl::event_code::NetworkEvent::EACMessage),
        143 => Some(rlnl::event_code::NetworkEvent::Taunt),
        144 => Some(rlnl::event_code::NetworkEvent::MachineFullHealth),
        145 => Some(rlnl::event_code::NetworkEvent::RequestSync),
        146 => Some(rlnl::event_code::NetworkEvent::BeginSync),
        147 => Some(rlnl::event_code::NetworkEvent::EndOfSync),
        148 => Some(rlnl::event_code::NetworkEvent::SyncTeamBaseCubes),
        150 => Some(rlnl::event_code::NetworkEvent::SyncEqualizerNotification),
        151 => Some(rlnl::event_code::NetworkEvent::PlayerQuitRequest),
        152 => Some(rlnl::event_code::NetworkEvent::PlayerQuitRequestComplete),
        153 => Some(rlnl::event_code::NetworkEvent::DamageCubeEffectOnly),
        154 => Some(rlnl::event_code::NetworkEvent::DamageCubeNoEffect),
        155 => Some(rlnl::event_code::NetworkEvent::DestroyCubeEffectOnly),
        156 => Some(rlnl::event_code::NetworkEvent::DestroyCubeNoEffect),
        157 => Some(rlnl::event_code::NetworkEvent::DestroyCubesFull),
        170 => Some(rlnl::event_code::NetworkEvent::LongPlayValue),
        171 => Some(rlnl::event_code::NetworkEvent::UpdateVotingAfterBattle),
        172 => Some(rlnl::event_code::NetworkEvent::CosmeticAction),
        _ => None,
    }
}

#[inline]
pub const fn i16_to_event_or_panic(num: i16) -> rlnl::event_code::NetworkEvent {
    match num {
        0 => rlnl::event_code::NetworkEvent::OnFailedToConnectToMasterServer,
        1 => rlnl::event_code::NetworkEvent::OnConnectingToLobbyServer,
        2 => rlnl::event_code::NetworkEvent::OnConnectedToLobbyServer,
        3 => rlnl::event_code::NetworkEvent::OnDisconnectingFromLobbyServer,
        4 => rlnl::event_code::NetworkEvent::OnDisconnectedFromLobbyServer,
        5 => rlnl::event_code::NetworkEvent::OnConnectedToServer,
        6 => rlnl::event_code::NetworkEvent::OnFailedToConnectToServer,
        7 => rlnl::event_code::NetworkEvent::OnConnectionLost,
        8 => rlnl::event_code::NetworkEvent::OnDisconnectedFromServer,
        9 => rlnl::event_code::NetworkEvent::OnConnectedToGameServer,
        10 => rlnl::event_code::NetworkEvent::OnServerStarted,
        11 => rlnl::event_code::NetworkEvent::OnServerStopped,
        12 => rlnl::event_code::NetworkEvent::OnPlayerConnectedToServer,
        13 => rlnl::event_code::NetworkEvent::OnPlayerDisconnectedFromServer,
        14 => rlnl::event_code::NetworkEvent::RequestRespawnPoint,
        15 => rlnl::event_code::NetworkEvent::FreeSpawnPoint,
        16 => rlnl::event_code::NetworkEvent::RequestTeamBaseModel,
        17 => rlnl::event_code::NetworkEvent::RequestCapturePoints,
        18 => rlnl::event_code::NetworkEvent::RequestEqualizerModel,
        19 => rlnl::event_code::NetworkEvent::TeamBase,
        20 => rlnl::event_code::NetworkEvent::RegisterCapturePoints,
        21 => rlnl::event_code::NetworkEvent::RegisterEqualizer,
        22 => rlnl::event_code::NetworkEvent::FreeRespawnPoint,
        23 => rlnl::event_code::NetworkEvent::PlayerIDs,
        24 => rlnl::event_code::NetworkEvent::SyncMachineCubes,
        25 => rlnl::event_code::NetworkEvent::MachineDestroyed,
        26 => rlnl::event_code::NetworkEvent::GameStarted,
        27 => rlnl::event_code::NetworkEvent::OnPlayerInputChanged,
        28 => rlnl::event_code::NetworkEvent::OnServerReceivedInputChange,
        29 => rlnl::event_code::NetworkEvent::ClientUnregistered,
        30 => rlnl::event_code::NetworkEvent::OnAnotherClientDisconnected,
        31 => rlnl::event_code::NetworkEvent::OnClientReconnected,
        32 => rlnl::event_code::NetworkEvent::DamageCube,
        33 => rlnl::event_code::NetworkEvent::FireWeaponEffect,
        35 => rlnl::event_code::NetworkEvent::FireMiss,
        36 => rlnl::event_code::NetworkEvent::MultipleFireMisses,
        37 => rlnl::event_code::NetworkEvent::CurrentGameTime,
        38 => rlnl::event_code::NetworkEvent::EndGame,
        39 => rlnl::event_code::NetworkEvent::GameWon,
        40 => rlnl::event_code::NetworkEvent::GameLost,
        41 => rlnl::event_code::NetworkEvent::GameWonBaseDestroyed,
        42 => rlnl::event_code::NetworkEvent::GameLostBaseDestroyed,
        43 => rlnl::event_code::NetworkEvent::BuffTeamPlayers,
        44 => rlnl::event_code::NetworkEvent::PlayerThreateningBase,
        45 => rlnl::event_code::NetworkEvent::TimeToGameStart,
        46 => rlnl::event_code::NetworkEvent::SetRespawnWaitingTime,
        47 => rlnl::event_code::NetworkEvent::TeamBaseState,
        48 => rlnl::event_code::NetworkEvent::TeamBaseCaptureStart,
        49 => rlnl::event_code::NetworkEvent::TeamBaseCaptureReset,
        50 => rlnl::event_code::NetworkEvent::TeamBaseCaptureStop,
        51 => rlnl::event_code::NetworkEvent::TeamBaseSectionComplete,
        52 => rlnl::event_code::NetworkEvent::TeamBaseFinalSectionComplete,
        53 => rlnl::event_code::NetworkEvent::TeamBaseInitialise,
        54 => rlnl::event_code::NetworkEvent::GetClientPings,
        55 => rlnl::event_code::NetworkEvent::SetClientPing,
        56 => rlnl::event_code::NetworkEvent::WarnPlayer,
        57 => rlnl::event_code::NetworkEvent::EnemySpotted,
        58 => rlnl::event_code::NetworkEvent::RemoteEnemySpotted,
        59 => rlnl::event_code::NetworkEvent::AssistBonusRequest,
        60 => rlnl::event_code::NetworkEvent::TeamBaseContested,
        61 => rlnl::event_code::NetworkEvent::AcquireRemoteAI,
        63 => rlnl::event_code::NetworkEvent::KillBonusRequest,
        64 => rlnl::event_code::NetworkEvent::EACRegisterToken,
        65 => rlnl::event_code::NetworkEvent::HeallingAssistBonusRequest,
        66 => rlnl::event_code::NetworkEvent::ProtectTeamMateBonusRequest,
        67 => rlnl::event_code::NetworkEvent::DefendTheBaseBonusRequest,
        68 => rlnl::event_code::NetworkEvent::DestroyCubesBonusRequest,
        69 => rlnl::event_code::NetworkEvent::DestroyHealCubesPointsAwarded,
        70 => rlnl::event_code::NetworkEvent::ConfirmedKill,
        71 => rlnl::event_code::NetworkEvent::BonusesFlushDone,
        72 => rlnl::event_code::NetworkEvent::AlignmentRectifierStarted,
        73 => rlnl::event_code::NetworkEvent::HealCubesBonusRequest,
        74 => rlnl::event_code::NetworkEvent::SetShieldState,
        75 => rlnl::event_code::NetworkEvent::TeamBaseLowHealth,
        76 => rlnl::event_code::NetworkEvent::AwardTeamBaseProtoniumDestroyedRequest,
        77 => rlnl::event_code::NetworkEvent::InitialiseGameStats,
        78 => rlnl::event_code::NetworkEvent::UpdateGameStats,
        79 => rlnl::event_code::NetworkEvent::MapPingEvent,
        80 => rlnl::event_code::NetworkEvent::SurrenderRequest,
        81 => rlnl::event_code::NetworkEvent::HealSelf,
        82 => rlnl::event_code::NetworkEvent::HealSelfResponse,
        83 => rlnl::event_code::NetworkEvent::SurrenderVoteStarted,
        84 => rlnl::event_code::NetworkEvent::SurrenderVoteCast,
        85 => rlnl::event_code::NetworkEvent::CurrentSurrenderVotes,
        86 => rlnl::event_code::NetworkEvent::SurrenderAccepted,
        87 => rlnl::event_code::NetworkEvent::SurrenderDeclined,
        88 => rlnl::event_code::NetworkEvent::SetSurrenderTimes,
        89 => rlnl::event_code::NetworkEvent::SetFinalGameScore,
        90 => rlnl::event_code::NetworkEvent::PitLeaderBoardUpdate,
        91 => rlnl::event_code::NetworkEvent::PitModeState,
        93 => rlnl::event_code::NetworkEvent::ValidateGameGuid,
        94 => rlnl::event_code::NetworkEvent::GameGuidValidated,
        95 => rlnl::event_code::NetworkEvent::PlayerInsideBase,
        97 => rlnl::event_code::NetworkEvent::ConfirmedAssist,
        98 => rlnl::event_code::NetworkEvent::LockOnNotification,
        99 => rlnl::event_code::NetworkEvent::LockOnNotificationBroadcast,
        100 => rlnl::event_code::NetworkEvent::ShieldSpawned,
        101 => rlnl::event_code::NetworkEvent::SpawnShield,
        102 => rlnl::event_code::NetworkEvent::BroadcastOpenShield,
        103 => rlnl::event_code::NetworkEvent::OpenShield,
        104 => rlnl::event_code::NetworkEvent::BroadcastInvisible,
        105 => rlnl::event_code::NetworkEvent::MakeInvisible,
        106 => rlnl::event_code::NetworkEvent::BroadcastVisible,
        107 => rlnl::event_code::NetworkEvent::MakeVisible,
        108 => rlnl::event_code::NetworkEvent::BroadcastActivateTeleportEffect,
        109 => rlnl::event_code::NetworkEvent::ActivateTeleportEffect,
        110 => rlnl::event_code::NetworkEvent::BroadcastActivateReadyEffect,
        111 => rlnl::event_code::NetworkEvent::ActivateReadyEffect,
        112 => rlnl::event_code::NetworkEvent::BroadcastSpawnEmpLocator,
        113 => rlnl::event_code::NetworkEvent::SpawnEmpLocator,
        114 => rlnl::event_code::NetworkEvent::BroadcastSpawnEmpMachineEffect,
        115 => rlnl::event_code::NetworkEvent::SpawnEmpMachineEffect,
        116 => rlnl::event_code::NetworkEvent::WeaponSelect,
        117 => rlnl::event_code::NetworkEvent::BroadcastWeaponSelect,
        118 => rlnl::event_code::NetworkEvent::HostAIs,
        119 => rlnl::event_code::NetworkEvent::HealAlly,
        120 => rlnl::event_code::NetworkEvent::HealAllyResponse,
        121 => rlnl::event_code::NetworkEvent::SelfDestructClassicMode,
        122 => rlnl::event_code::NetworkEvent::GameModeSettings,
        123 => rlnl::event_code::NetworkEvent::TeamDeathMatchState,
        124 => rlnl::event_code::NetworkEvent::ClientDisconnecting,
        125 => rlnl::event_code::NetworkEvent::SendBonus,
        126 => rlnl::event_code::NetworkEvent::TestConnection,
        127 => rlnl::event_code::NetworkEvent::MachineDestroyedConfirmed,
        128 => rlnl::event_code::NetworkEvent::EnergyModuleActivated,
        129 => rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
        130 => rlnl::event_code::NetworkEvent::RequestLoadingProgressAllUsers,
        131 => rlnl::event_code::NetworkEvent::LoadingComplete,
        132 => rlnl::event_code::NetworkEvent::GameAborted,
        134 => rlnl::event_code::NetworkEvent::SendDamagedByEnemyShield,
        135 => rlnl::event_code::NetworkEvent::DamagedByEnemyShield,
        136 => rlnl::event_code::NetworkEvent::EqualizerNotification,
        137 => rlnl::event_code::NetworkEvent::CapturePointProgress,
        138 => rlnl::event_code::NetworkEvent::CapturePointNotification,
        140 => rlnl::event_code::NetworkEvent::RadarModuleActivated,
        141 => rlnl::event_code::NetworkEvent::RemoteRadarModuleActivated,
        142 => rlnl::event_code::NetworkEvent::EACMessage,
        143 => rlnl::event_code::NetworkEvent::Taunt,
        144 => rlnl::event_code::NetworkEvent::MachineFullHealth,
        145 => rlnl::event_code::NetworkEvent::RequestSync,
        146 => rlnl::event_code::NetworkEvent::BeginSync,
        147 => rlnl::event_code::NetworkEvent::EndOfSync,
        148 => rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
        150 => rlnl::event_code::NetworkEvent::SyncEqualizerNotification,
        151 => rlnl::event_code::NetworkEvent::PlayerQuitRequest,
        152 => rlnl::event_code::NetworkEvent::PlayerQuitRequestComplete,
        153 => rlnl::event_code::NetworkEvent::DamageCubeEffectOnly,
        154 => rlnl::event_code::NetworkEvent::DamageCubeNoEffect,
        155 => rlnl::event_code::NetworkEvent::DestroyCubeEffectOnly,
        156 => rlnl::event_code::NetworkEvent::DestroyCubeNoEffect,
        157 => rlnl::event_code::NetworkEvent::DestroyCubesFull,
        170 => rlnl::event_code::NetworkEvent::LongPlayValue,
        171 => rlnl::event_code::NetworkEvent::UpdateVotingAfterBattle,
        172 => rlnl::event_code::NetworkEvent::CosmeticAction,
        _ => panic!("Invalid rlnl event code"),
    }
}
