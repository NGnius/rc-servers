mod engine;
pub(self) use engine::{CustomGameLogic, RlnlPacket};

mod messages;
pub use messages::GameMessage;

mod generic;
pub(self) use generic::GenericGamemodeEngine;

mod aggregate;
pub use aggregate::GameMatches;

mod countdown;

pub mod modes;

mod timer;

pub const CHANNEL_BOUND: usize = 16;
