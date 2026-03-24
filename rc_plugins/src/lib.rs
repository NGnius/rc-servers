pub mod chat;
pub mod vehicle_validation;
pub mod team_selection;

pub trait Plugin: Send + Sync {
    fn self_check(&self) -> bool {
        true
    }
}
