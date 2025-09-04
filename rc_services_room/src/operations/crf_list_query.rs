use polariton_server::operations::{Operation, OperationCode};
use polariton::operation::{Typed, ParameterTable};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 86;

const FILTERS_PARAM_KEY: u8 = 92;
const ITEMS_PARAM_KEY: u8 = 93;

async fn do_handling(params: ParameterTable<()>, _user: &crate::UserTy, factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Bytes(filters)) = params.remove(&FILTERS_PARAM_KEY) {
        let mut cursor = std::io::Cursor::new(filters.vec);
        let filters = crate::data::crf::ShopItemListFilters::parse(&mut cursor).map_err(|e| {
            log::error!("Failed to parse factory item query: {}", e);
            oj_rc_core::data::error_codes::WebServicesError::UnexpectedError as i16
        })?;
        let vehicles = factory.list(filters.into_core()).await.map_err(|e| {
            log::error!("Failed to retrieve vehicles from factory: {}", e);
            oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16
        })?;
        let vehicles: Vec<_> = vehicles.into_iter().map(crate::data::crf::ItemResult::from).collect();
        params.insert(ITEMS_PARAM_KEY, crate::data::crf::ItemResult::as_transmissible(&vehicles));
    }
    Ok(params.into())
}

pub struct CrfItemListProvider {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
}

#[async_trait::async_trait]
impl Operation<()> for CrfItemListProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<()>, user: &Self::User) -> polariton::operation::OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, &self.factory).await)
    }
}

impl OperationCode for CrfItemListProvider {
    fn op_code() -> u8 {
        CODE
    }
}

pub(super) fn crf_item_list_query_provider(factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> CrfItemListProvider {
    CrfItemListProvider {
        factory: factory.to_owned(),
    }
}
