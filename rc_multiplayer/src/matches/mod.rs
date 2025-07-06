mod engine;
pub use engine::GamemodeEngine;

mod messages;
pub use messages::GameMessage;

mod generic;
pub(self) use generic::GenericGamemodeEngine;

mod aggregate;
pub use aggregate::GameMatches;

pub const CHANNEL_BOUND: usize = 16;
