mod engine;
 use engine::{CustomGameLogic, RlnlPacket};

mod messages;
pub use messages::GameMessage;

mod generic;
 use generic::GenericGamemodeEngine;

mod aggregate;
pub use aggregate::GameMatches;

mod countdown;

pub mod modes;

mod timer;

 mod fake;

pub const CHANNEL_BOUND: usize = 16;
