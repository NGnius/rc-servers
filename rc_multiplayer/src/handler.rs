pub struct LnlEventHandler {
    event_handlers: std::collections::HashMap<u16, Box<dyn super::EventCodeHandler>>,
}

impl LnlEventHandler {
    pub async fn new(init_ctx: &crate::InitConfig) -> Self {
        let mut event_handlers = std::collections::HashMap::new();
        // init event code handlers
        Self {
            event_handlers
        }
    }
}

#[async_trait::async_trait]
impl literustlib_server::EventHandler for LnlEventHandler {
    type PacketData = super::PacketData;
    type UserData = super::UserData;

    async fn on_receive(&self, data: Self::PacketData, _header: &literustlib::packet::Header, peer: &std::sync::Arc< literustlib_server::Connection<Self::PacketData>>, user: &Self::UserData, sender: &literustlib_server::DataSender<Self::PacketData>) {
        log::debug!("Got event {:?} (len: {}) from connection id {}", data.variant, data.data.len(), peer.id());
        if let Some(handler) = self.event_handlers.get(&(data.variant as u16)) {
            handler.handle(&data.data, peer, user, sender).await;
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Unsupported event variant {:?} ({}), pls fix!!!\n {:?}", data.variant, data.variant as u16, &data.data[..]);
            }
            #[cfg(not(debug_assertions))]
            {
                log::warn!("Unsupported event variant {:?} ({}), pls fix!!!", data.variant, data.variant as u16);
            }
        }
    }

    async fn on_connect_start(&self, addr: &core::net::SocketAddr, key: String, peer: &std::sync::Arc< literustlib_server::Connection<Self::PacketData>>) -> Option<Self::UserData> {
        log::debug!("New connection started from {}!!! (key:{}, id:{})", addr, key, peer.id());
        //let mut buf = Vec::new();
        //literustlib::packet::Packet::with_data(literustlib::packet::Property::Reliable, &[9, 0, 0, 0, 0, 0]).dump(&mut buf).unwrap_or_default();
        //socket.send_to(&buf, addr).await.unwrap_or_default();
        Some(())
    }

    async fn on_connect_done(&self, peer: &std::sync::Arc< literustlib_server::Connection<Self::PacketData>>, _user: &Self::UserData, sender: &literustlib_server::DataSender<Self::PacketData>) {
        log::debug!("New connection completed (id:{})", peer.id());
        //let mut buf = Vec::new();
        //literustlib::packet::Packet::with_data(literustlib::packet::Property::Reliable, &[9, 0, 0, 0, 0, 0]).dump(&mut buf).unwrap_or_default();
        //socket.send_to(&buf, addr).await.unwrap_or_default();
        if let Err(e) = sender.send_to(bytes::Bytes::from_static(&[49, 0, 9, 0, 0, 0]), literustlib::packet::Property::Reliable, peer).await {
            log::error!("Failed to send rlnl OnConnectedToGameServer event: {}", e);
        }

    }
}

#[derive(Debug)]
pub struct EventData {
    pub message_ty: crate::data::MessageType,
    pub variant: rlnl::event_code::NetworkEvent,
    pub data_size: u16,
    pub data: bytes::Bytes,
}

impl literustlib::packet::PacketData for EventData {
    fn parse(bytes: bytes::Bytes, _header: &literustlib::packet::Header) -> std::io::Result<Self> {
        if bytes.len() >= 6 {
            let data = bytes.slice(6..);
            let slice: &[u8] = &bytes[0..6];
            let net_message_num = i16::from_le_bytes([bytes[0], bytes[1]]);
            let net_message_type = crate::data::MessageType::from_i16(net_message_num).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Unsupported, format!("Unsupported message type {}", net_message_num)))?;
            let event_code = i16::from_le_bytes([bytes[2], bytes[3]]);
            let data_size = u16::from_le_bytes([bytes[4], bytes[5]]);
            if let Some(event_variant) = i16_to_event(event_code) {
                Ok(Self {
                    message_ty: net_message_type,
                    variant: event_variant,
                    data_size,
                    data,
                })
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid packet code"))
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Packet data is not long enough"))
        }

    }

    fn dump(&self) -> Vec<u8> {
        use std::io::Write;
        let mut buf = Vec::new();
        buf.write_all(&(self.variant as u16).to_be_bytes()).unwrap();
        buf.write_all(&self.data).unwrap();
        buf
    }
}

fn i16_to_event(num: i16) -> Option<rlnl::event_code::NetworkEvent> {
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
