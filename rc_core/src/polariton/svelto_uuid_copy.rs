use polariton_server::operations::OperationModifier;

pub struct OpIdCopy;

impl OpIdCopy {
    const SERVICE_MAPPING_KEY: u8 = 0;
}

impl <C: Clone + Send + Sync + 'static> OperationModifier<C> for OpIdCopy {
    fn after(&self, req: &mut polariton::operation::OperationRequest<C>, resp: &mut polariton::operation::OperationResponse<C>, _flags: &mut u8) {
        if resp.params.get(&Self::SERVICE_MAPPING_KEY).is_none() {
            if let Some(svelto_service_id) = req.params.get(&Self::SERVICE_MAPPING_KEY) {
                resp.params.insert(Self::SERVICE_MAPPING_KEY, svelto_service_id.to_owned());
            }
        }

    }
}
