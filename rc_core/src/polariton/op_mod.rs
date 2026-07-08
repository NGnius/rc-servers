use polariton_server::operations::OperationModifier;

pub type OpModImpl = RcOpModifier;

pub struct RcOpModifier {
    settings: crate::persist::PolaritonSettings,
    unique_param_value: std::sync::atomic::AtomicI64,
}

impl RcOpModifier {
    const SERVICE_MAPPING_KEY: u8 = 0;
    const SAFE_UNIQUE_RESPONSE_PARAM: u8 = 255;

    pub(crate) fn new(settings: crate::persist::PolaritonSettings) -> Self {
        let now = chrono::Utc::now();
        Self {
            settings,
            unique_param_value: std::sync::atomic::AtomicI64::new(
                now.timestamp_nanos_opt()
                    .unwrap_or(now.timestamp_millis())
            ),
        }
    }
}

impl <C: Clone + Send + Sync + 'static> OperationModifier<C> for RcOpModifier {
    fn after(&self, req: &mut polariton::operation::OperationRequest<C>, resp: &mut polariton::operation::OperationResponse<C>, flags: &mut u8) {
        if resp.params.get(&Self::SERVICE_MAPPING_KEY).is_none() {
            if let Some(svelto_service_id) = req.params.get(&Self::SERVICE_MAPPING_KEY) {
                resp.params.insert(Self::SERVICE_MAPPING_KEY, svelto_service_id.to_owned());
            }
        }
        if self.settings.force_encrypt_responses {
            *flags |= 0x80;
        }
        if self.settings.add_unique_response_param {
            #[cfg(debug_assertions)]
            if resp.params.get(&Self::SAFE_UNIQUE_RESPONSE_PARAM).is_some() {
                log::warn!("Smashed param {} of polariton response {}", Self::SAFE_UNIQUE_RESPONSE_PARAM, resp.code);
            }
            let new_val = self.unique_param_value.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            resp.params.insert(Self::SAFE_UNIQUE_RESPONSE_PARAM, polariton::operation::Typed::Long(new_val));
        }
    }
}
