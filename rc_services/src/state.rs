use std::sync::Arc;

pub struct State {
    pub crypto: Box<Arc<dyn polariton::packet::Cryptographer>>,
}

impl State {
    pub fn new(c: Box<Arc<dyn polariton::packet::Cryptographer>>) -> Self {
        Self {
            crypto: c,
        }
    }

    pub fn binrw_args(&self) -> polariton::packet::WriteArgs {
        Some(self.crypto.clone())
    }
}
