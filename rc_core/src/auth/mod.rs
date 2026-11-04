mod token;
pub use token::{Token, LoginMethod, FediToken};

pub fn user_agent() -> String {
    format!("OpenJam ({}/{})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
