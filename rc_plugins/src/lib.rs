pub mod chat;
pub mod vehicle_validation;

pub trait Plugin: Send + Sync {
    fn self_check(&self) -> bool {
        true
    }
}
