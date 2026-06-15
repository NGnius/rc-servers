pub mod chat;
pub mod vehicle_validation;
pub mod team_selection;
pub mod vehicle_import;

pub trait Plugin: Send + Sync {
    fn self_check(&self) -> bool {
        true
    }
}
