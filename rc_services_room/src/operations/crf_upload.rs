use polariton_server::operations::{Operation, OperationCode};
use polariton::operation::{Typed, ParameterTable};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 96;

const DATA_PARAM_KEY: u8 = 101; // in; dict
const VERSION_PARAM_KEY: u8 = 99; // in; str
const SUCCESS_PARAM_KEY: u8 = 103; // out; bool

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy, factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Str(version)) = params.remove(&VERSION_PARAM_KEY) {
        if let Some(Typed::Dict(data)) = params.remove(&DATA_PARAM_KEY) {
            let upload_info = crate::data::crf::UploadData::from_transmissibles(version.string, data)?;
            let user_info = user.user()?;
            let prepared = user_info.prepare_factory_upload(upload_info.into_core()).await?;
            let success = factory.upload(prepared).await.map_err(|e| {
                log::error!("Failed to upload to factory: {}", e);
                oj_rc_core::data::error_codes::WebServicesError::UnexpectedError as i16
            })?;
            params.insert(SUCCESS_PARAM_KEY, Typed::Bool(success));
        }
    }
    Ok(params.into())
}

pub struct CrfUploadProvider {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
}

#[async_trait::async_trait]
impl Operation<()> for CrfUploadProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<()>, user: &Self::User) -> polariton::operation::OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, &self.factory).await)
    }
}

impl OperationCode for CrfUploadProvider {
    fn op_code() -> u8 {
        CODE
    }
}

pub(super) fn crf_upload_provider(factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> CrfUploadProvider {
    CrfUploadProvider {
        factory: factory.to_owned(),
    }
}
