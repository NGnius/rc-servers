use polariton::operation::{ParameterTable, OperationResponse};
use polariton_server::operations::{Operation, OperationCode};

pub struct SimpleChatFunc<const CODE: u8, U: Send + Sync, F: (Fn(ParameterTable<C>, &U, &crate::state::ChatImpl) -> Result<ParameterTable<C>, i16>) + Send + Sync, C: Send + Sync + 'static = ()> {
    _user_ty: std::marker::PhantomData<U>,
    _custom_ty: std::marker::PhantomData<C>,
    chat: crate::state::ChatImpl,
    func: F,
}

impl <C: Send + Sync + 'static, const CODE: u8, U: Send + Sync, F: (Fn(ParameterTable<C>, &U, &crate::state::ChatImpl) -> Result<ParameterTable<C>, i16>) + Send + Sync> SimpleChatFunc<CODE, U, F, C> {
    pub fn new(f: F, chat: crate::state::ChatImpl) -> Self {
        Self {
            _user_ty: std::marker::PhantomData,
            _custom_ty: std::marker::PhantomData,
            chat,
            func: f,
        }
    }
}

impl <C: Send + Sync + 'static, const CODE: u8, U: Send + Sync, F: (Fn(ParameterTable<C>, &U, &crate::state::ChatImpl) -> Result<ParameterTable<C>, i16>) + Send + Sync> Operation<C> for SimpleChatFunc<CODE, U, F, C> {
    type User = U;

    fn handle(&self, p: polariton::operation::ParameterTable<C>, u: &Self::User) -> OperationResponse<C> {
        match (self.func)(p, u, &self.chat) {
            Ok(p_out) => {
                OperationResponse {
                    code: CODE,
                    return_code: 0,
                    message: polariton::operation::Typed::Null,
                    params: p_out,
                }
            },
            Err(e_code) => {
                OperationResponse {
                    code: CODE,
                    return_code: e_code,
                    message: polariton::operation::Typed::Null,
                    params: std::collections::HashMap::new().into(),
                }
            }
        }

    }
}

impl <C: Send + Sync + 'static, const CODE: u8, U: Send + Sync, F: (Fn(ParameterTable<C>, &U, &crate::state::ChatImpl) -> Result<ParameterTable<C>, i16>) + Send + Sync> OperationCode for SimpleChatFunc<CODE, U, F, C> {
    fn op_code() -> u8 {
        CODE
    }
}
