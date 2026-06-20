#![allow(dead_code)]
use polariton::operation::Typed;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CustomGameInviteCode {
    NoInvite = 0,
    PendingInvite = 1,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SessionCreateResponseCode {
    SessionCreated = 0,
    AlreadyInSession = 1,
    SessionCreateError = 2,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AdjustCustomGameConfigResponseCode {
    Success = 0,
    NotInSession = 1,
    AdjustmentRejected = 2,
    // 3, 4, 5 also exist but seem equivalent to 2
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SessionLeaveResponseCode {
    Success = 0,
    NotInSession = 1,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum KickResponseCode {
    SessionNoLongerExists = 0,
    KickTargetIsNotInsession = 1,
    UserIsNotSessionLeader = 2,
    UserRemovedFromSession = 3,
    ErrorKickingFromSession = 4
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SessionRetrieveResponse {
    UserNotInAnySession = 0,
    SessionRetrieved = 1,
    PlayerIsInvitedOnly = 2,
    ErrorRetrievingSession = 3,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum InviteToCustomGameResponseCode {
    UserIsNotInSession = 0,
    UserIsNotSessionLeader = 1,
    InviteeHasAlreadyBeenInvited = 2,
    UserIsNotOnline = 3,
    UserInvited = 4,
    ErrorDispatchingMessage = 5,
    InviteeIsInAnotherCustomGame = 6,
    InviteeIsAlreadyInvitedToAnotherCustomGame = 7,
    UserDoesNotExist = 8,
    UserOnlyAcceptsInvitesFromFriendsAndClanmates = 9,
    UserBlockedYou = 10,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum InviteReplyCustomGameResponseCode {
    Success = 1,
    Failure2 = 2,
    UserIsNotInSession = 4,
    UserIsNoLongerInvited = 5,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CheckCanJoinQueueResponseCode {
    UserNotInSession0 = 0,
    OnlyOneAllowed = 1,
    Unbalanced = 2,
    AlreadyInBattle = 3,
    Ok = 4,
    UserNotInSession5 = 5,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ChangeTeamResponseCode {
    UnknownFail0 = 0,
    UserIsNotSessionLeader = 1,
    UnknownFail2 = 2,
    UnknownFail3 = 3,
    Success = 4,
    UnknownFail5 = 5,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PlayerSessionStatus {
    Unknown = 0,
    Ready = 1,
    Queuing = 2,
    InBattle = 3,
}

impl PlayerSessionStatus {
    #[inline]
    pub fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::Unknown),
            1 => Some(Self::Ready),
            2 => Some(Self::Queuing),
            3 => Some(Self::InBattle),
            _ => None,
        }
    }

    #[inline]
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct Session {
    pub leader: String,
    pub session: String,
    pub members: Vec<String>,
    pub members_display_name: Vec<String>,
    pub invited: Vec<String>,
    pub team_b_members: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
    pub avatar_info: std::collections::HashMap<String, oj_rc_core::data::player_data::AvatarInfo>,
    pub player_session_state: std::collections::HashMap<String, PlayerSessionStatus>,
}

impl Session {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("Leader".into()), Typed::Str(self.leader.clone().into())),
            (Typed::Str("SessionID".into()), Typed::Str(self.session.clone().into())),
            (Typed::Str("Members".into()), Typed::HashMap(self.members.iter()
                .enumerate()
                .map(|(i, pub_id)| (Typed::Int(i as _), Typed::Str(pub_id.into())))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
            (Typed::Str("MembersDisplayName".into()), Typed::HashMap(self.members_display_name.iter()
                .enumerate()
                .map(|(i, display_name)| (Typed::Int(i as _), Typed::Str(display_name.into())))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
            (Typed::Str("Invited".into()), Typed::HashMap(self.invited.iter()
                .enumerate()
                .map(|(i, pub_id)| (Typed::Int(i as _), Typed::Str(pub_id.into())))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
            (Typed::Str("TeamBMembers".into()), Typed::HashMap(self.team_b_members.iter()
                .enumerate()
                .map(|(i, pub_id)| (Typed::Int(i as _), Typed::Str(pub_id.into())))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
            (Typed::Str("Config".into()), Typed::HashMap(self.config.iter()
                .map(|(key, val)| (Typed::Str(key.into()), Typed::Str(val.into())))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
            (Typed::Str("AvatarInfo".into()), Typed::HashMap(self.avatar_info.iter()
                .map(|(key, val)| (Typed::Str(key.into()), val.as_transmissible()))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
            (Typed::Str("PlayerSessionState".into()), Typed::HashMap(self.player_session_state.iter()
                .map(|(key, val)| (Typed::Str(key.into()), Typed::Int(*val as _)))
                .collect::<Vec<(_, _)>>()
                .into()
            )),
        ].into())
    }
}

/// not to be confused with the event which contains the same data
/// ... just serialized differently (to be difficult?)
pub struct CustomGameInvite {
    pub inviter_public_id: String,
    pub inviter_display_name: String,
    pub session: String,
    pub avatar_id: Option<i32>,
    pub invited_to_team_b: bool,
}

impl CustomGameInvite {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (polariton::operation::Typed::Str("SenderName".into()), polariton::operation::Typed::Str(self.inviter_public_id.clone().into())),
            (polariton::operation::Typed::Str("SenderDisplayName".into()), polariton::operation::Typed::Str(self.inviter_display_name.clone().into())),
            (polariton::operation::Typed::Str("SessionGUID".into()), polariton::operation::Typed::Str(self.session.clone().into())),
            (polariton::operation::Typed::Str("UseCustomAvatar".into()), polariton::operation::Typed::Bool(self.avatar_id.is_none())),
            (polariton::operation::Typed::Str("AvatarId".into()), polariton::operation::Typed::Int(self.avatar_id.unwrap_or(0))),
            (polariton::operation::Typed::Str("IsInvitedToTeamB".into()), polariton::operation::Typed::Bool(self.invited_to_team_b)),
        ].into())
    }
}
