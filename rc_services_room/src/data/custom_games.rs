#![allow(dead_code)]

pub use oj_rc_core::data::game_mode::GameMode;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CustomGameInviteCode {
    NoInvite = 0,
    PendingInvite = 1,
}
