use polariton_server::operations::{Operation, OperationCode};

pub struct UserFlagsTeller;

impl UserFlagsTeller {
    const REMOVE_OBSOLETE_CUBES_KEY: u8 = 113;
    const REMOVE_UNOWNED_CUBES_KEY: u8 = 114;
    const REWARD_TITLE_KEY: u8 = 115;
    const REWARD_BODY_KEY: u8 = 116;
    const REFUND_OBSOLETE_CUBES_KEY: u8 = 117;
    const CUBES_ARE_REPLACED_KEY: u8 = 118;
    const NEW_USER_KEY: u8 = 119;
    const AB_TEST_KEY: u8 = 166;
    const AB_GROUP_KEY: u8 = 167;
}

impl <C> Operation<C> for UserFlagsTeller {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, _: polariton::operation::ParameterTable<C>, _: &mut Self::State, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut resp_params = std::collections::HashMap::new();
        resp_params.insert(Self::REMOVE_OBSOLETE_CUBES_KEY, polariton::operation::Typed::Bool(false.into()));
        resp_params.insert(Self::REMOVE_UNOWNED_CUBES_KEY, polariton::operation::Typed::Bool(false.into()));
        resp_params.insert(Self::REWARD_TITLE_KEY, polariton::operation::Typed::Str("Yay a reward!".into()));
        resp_params.insert(Self::REWARD_BODY_KEY, polariton::operation::Typed::Str("I love you very much so here's nothing as a reward.".into()));
        resp_params.insert(Self::REFUND_OBSOLETE_CUBES_KEY, polariton::operation::Typed::Bool(false.into()));
        resp_params.insert(Self::CUBES_ARE_REPLACED_KEY, polariton::operation::Typed::Bool(false.into()));
        resp_params.insert(Self::NEW_USER_KEY, polariton::operation::Typed::Bool(false.into()));
        resp_params.insert(Self::AB_TEST_KEY, polariton::operation::Typed::Str("".into()));
        resp_params.insert(Self::AB_GROUP_KEY, polariton::operation::Typed::Str("".into()));
        polariton::operation::OperationResponse {
            code: 105,
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params: resp_params.into(),
        }
    }
}

impl OperationCode for UserFlagsTeller {
    fn op_code() -> u8 {
        105
    }
}
