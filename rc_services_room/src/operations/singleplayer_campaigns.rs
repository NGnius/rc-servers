use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const CAMPAIGNS_BYTES_PARAM_KEY: u8 = 64; // list of bytes (serialised data)
const CAMPAIGNS_WAVES_PARAM_KEY: u8 = 70; // hashtable
const CAMPAIGNS_VERSIONS_PARAM_KEY: u8 = 69; // hashtable

pub(super) fn singleplayer_campaigns_provider() -> SimpleFunc<65, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        // TODO implement serialisation of Campaign[] properly
        params.insert(CAMPAIGNS_BYTES_PARAM_KEY, Typed::Bytes(vec![0u8, 0u8, 0u8, 0u8].into())); // first 4 bytes are i32 for length of the rest
        params.insert(CAMPAIGNS_WAVES_PARAM_KEY, Typed::HashMap(vec![].into()));
        params.insert(CAMPAIGNS_VERSIONS_PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("CurrentVersionNumber".into()), Typed::Int(0)),
            (Typed::Str("LockedCampaignsInfo".into()), Typed::HashMap(vec![
                (Typed::Str("0".into()), Typed::Bool(false.into()))
            ].into())),
        ].into()));
        Ok(params.into())
    })
}
