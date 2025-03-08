const SERDES_SERDES_CTX_REF: &'static polariton::serdes::SerdesContext<(), polariton::serdes::NoCustomSerdes> = &polariton::serdes::SerdesContext::default_const();

pub struct State {
    pub crypto: polariton_auth::CryptoImpl,
}

impl State {
    pub fn new(c: polariton_auth::CryptoImpl) -> Self {
        Self {
            crypto: c,
        }
    }

    pub fn serdes_ctx(&self) -> polariton::packet::SerdesContext<'_, (), polariton::serdes::NoCustomSerdes> {
        polariton::packet::SerdesContext::new(SERDES_SERDES_CTX_REF, &self.crypto)
    }
}
