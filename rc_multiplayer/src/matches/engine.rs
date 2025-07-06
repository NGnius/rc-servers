pub trait GamemodeEngine: Send + Sync {
    fn is_complete(&self) -> bool;
}
