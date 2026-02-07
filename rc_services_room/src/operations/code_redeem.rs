use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 107;

const CODE_NAME_PARAM_KEY: u8 = 122; // str; in
const SUCCESS_PARAM_KEY: u8 = 123; // bool; out
const RESULT_CODE_PARAM_KEY: u8 = 124; // int; out
const IS_SERIAL_PARAM_KEY: u8 = 125; // bool; out
const VALUE_PARAM_KEY: u8 = 126; // float; out
const PROMO_ID_PARAM_KEY: u8 = 127; // str; out
const CUBES_AWARDED_PARAM_KEY: u8 = 128; // json as str; out
const MSG_PARAM_KEY: u8 = 133; // str; out
const BUNDLE_ID_PARAM_KEY: u8 = 208; // str; out
const ROBOPASS_PARAM_KEY: u8 = 4; // bool; out
const PAID_CURRENCY_PARAM_KEY: u8 = 86; // long; out

#[allow(dead_code)]
#[repr(u8)]
enum PromoResultCode {
    Success = 0,
    InvalidBundleId = 1,
    InvalidPromotionId = 2,
    Expired = 3,
    NotStarted = 4,
    AlreadyAwarded = 5,
    Consumed = 6,
    BundleIdAlreadyAwarded = 7
}

pub(super) struct PromoCodeApplier {
    code_map: std::collections::HashMap<String, oj_rc_core::persist::config::PromoCode>,
}

#[async_trait::async_trait]
impl SimpleOperation<()> for PromoCodeApplier {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable, user: &Self::User) -> Result<ParameterTable, SimpleOpError> {
        if let Some(Typed::Str(promo_code)) = params.remove(&CODE_NAME_PARAM_KEY) {
            let user_info = user.user()?;
            if let Some(code_info) = self.code_map.get(&promo_code.string) {
                if !code_info.is_repeatable {
                    if !user_info.mark_code_redeemed(promo_code.string.clone()).await? {
                        params.insert(SUCCESS_PARAM_KEY, Typed::Bool(false));
                        params.insert(RESULT_CODE_PARAM_KEY, Typed::Int(PromoResultCode::AlreadyAwarded as _));
                        params.insert(IS_SERIAL_PARAM_KEY, Typed::Bool(false));
                        params.insert(VALUE_PARAM_KEY, Typed::Float(0.0));
                        params.insert(PROMO_ID_PARAM_KEY, Typed::Str(promo_code.clone()));
                        params.insert(CUBES_AWARDED_PARAM_KEY, Typed::Str("{}".into()));
                        params.insert(MSG_PARAM_KEY, Typed::Str("".into()));
                        params.insert(BUNDLE_ID_PARAM_KEY, Typed::Str(code_info.bundle_id.clone().into()));
                        params.insert(ROBOPASS_PARAM_KEY, Typed::Bool(false));
                        params.insert(PAID_CURRENCY_PARAM_KEY, Typed::Long(0));
                        log::debug!("Code \"{}\" not redeemed by {} (code already redeemed)", promo_code.string, user_info.public_id());
                        return Ok(params);
                    }
                }
                let result = user_info.apply_purchase(&code_info.transaction).await?;
                params.insert(SUCCESS_PARAM_KEY, Typed::Bool(result.success));
                params.insert(RESULT_CODE_PARAM_KEY, Typed::Int(PromoResultCode::Success as _));
                params.insert(IS_SERIAL_PARAM_KEY, Typed::Bool(code_info.is_serial));
                params.insert(VALUE_PARAM_KEY, Typed::Float(code_info.value));
                params.insert(PROMO_ID_PARAM_KEY, Typed::Str(code_info.promo_id.clone().into()));
                params.insert(CUBES_AWARDED_PARAM_KEY, Typed::Str(serde_json::to_string(&result.cube_awards).unwrap().into()));
                params.insert(MSG_PARAM_KEY, Typed::Str(code_info.message.clone().unwrap_or_default().into()));
                params.insert(BUNDLE_ID_PARAM_KEY, Typed::Str(code_info.bundle_id.clone().into()));
                params.insert(ROBOPASS_PARAM_KEY, Typed::Bool(result.robopass_award));
                params.insert(PAID_CURRENCY_PARAM_KEY, Typed::Long(result.paid_currency_award));
                log::debug!("Code \"{}\" redeemed by {} (success? {}) {} rewards", promo_code.string, user_info.public_id(), result.success, code_info.transaction.gives.len());
            } else {
                params.insert(SUCCESS_PARAM_KEY, Typed::Bool(false));
                params.insert(RESULT_CODE_PARAM_KEY, Typed::Int(PromoResultCode::InvalidPromotionId as _));
                params.insert(IS_SERIAL_PARAM_KEY, Typed::Bool(false));
                params.insert(VALUE_PARAM_KEY, Typed::Float(0.0));
                params.insert(PROMO_ID_PARAM_KEY, Typed::Str(promo_code.clone()));
                params.insert(CUBES_AWARDED_PARAM_KEY, Typed::Str("{}".into()));
                params.insert(MSG_PARAM_KEY, Typed::Str("".into()));
                params.insert(BUNDLE_ID_PARAM_KEY, Typed::Str("RE_bundle_id_01".into()));
                params.insert(ROBOPASS_PARAM_KEY, Typed::Bool(false));
                params.insert(PAID_CURRENCY_PARAM_KEY, Typed::Long(0));
                log::debug!("Code \"{}\" not redeemed by {} (code not found)", promo_code.string, user_info.public_id());
            }
        }
        Ok(params)
    }
}

pub(super) fn code_redeem_provider(conf: &oj_rc_core::ConfigImpl) -> SimpleOpImpl<(), crate::UserTy, PromoCodeApplier> {
    SimpleOpImpl::new(PromoCodeApplier {
        code_map: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::promo_codes(conf),
    })
}
