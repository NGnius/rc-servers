use polariton_server::operations::{Operation, OperationCode};
use polariton::{operation::{Dict, Typed}, serdes::TypePrefix};
use rc_core::ConfigProvider;

const PARAM_KEY: u8 = 16;

pub struct CubeInventoryProvider {
    cube_ids: Vec<u32>,
}

impl <C: Send + 'static> Operation<C> for CubeInventoryProvider {
    type User = crate::UserTy;

    fn handle(&self, params: polariton::operation::ParameterTable<C>, _user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Int, // int
            val_ty: TypePrefix::Int, // int
            items: self.cube_ids.iter().map(|id| (Typed::Int(*id as _), Typed::Int(1))).collect()}));
        polariton::operation::OperationResponse {
            code: Self::op_code(),
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params: params.into(),
        }
    }
}

impl OperationCode for CubeInventoryProvider {
    fn op_code() -> u8 {
        16
    }
}

pub(super) fn cube_inv_provider<'a>(cubes: &'a rc_core::ConfigImpl) -> CubeInventoryProvider {
    let cube_ids = <rc_core::ConfigImpl as ConfigProvider<()>>::ids(cubes);
    CubeInventoryProvider { cube_ids }
}
