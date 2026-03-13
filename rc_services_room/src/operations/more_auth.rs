use polariton::operation::Typed;
use polariton_server::operations::{Operation, OperationCode};

pub struct MoreLobbyAuth {
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

pub fn more_auth_provider(mesh: &std::sync::Arc<crate::user_service::UserMesh>) -> MoreLobbyAuth {
    MoreLobbyAuth {
        mesh: mesh.to_owned()
    }
}

impl MoreLobbyAuth {
    const AUTH_PAYLOAD_KEY: u8 = 245;
}

#[async_trait::async_trait]
impl <C: Send + 'static> Operation<C> for MoreLobbyAuth {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<C>, user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let params_dict = params.to_dict();
        if let Some(Typed::Str(auth_payload)) = params_dict.get(&Self::AUTH_PAYLOAD_KEY) {
            //let mut write_lock = user.write().unwrap();
            if user.update_with_auth(&auth_payload.string).await {
                let user_info = user.user().unwrap();
                if user_info.is_banned() {
                    return polariton::operation::OperationResponse {
                        code: Self::op_code(),
                        return_code: oj_rc_core::data::error_codes::WebServicesError::Banned as i16,
                        message: polariton::operation::Typed::Null,
                        params: polariton::operation::ParameterTable::with_capacity(0),
                    }
                } else {
                    match user_info.webservice_listener().await {
                        Ok(listener) => {
                            self.mesh.add_user(
                                user_info.public_id().to_owned(),
                                user.event_sender().to_owned().downgrade(),
                            ).await;
                            crate::ONLINE_USERS.store(self.mesh.user_count().await as u64, std::sync::atomic::Ordering::SeqCst);
                            crate::update_status(user_info.as_ref().as_ref()).await;
                            let mut resp_params = std::collections::HashMap::with_capacity(1);
                            resp_params.insert(Self::AUTH_PAYLOAD_KEY, polariton::operation::Typed::Byte(0));
                            crate::events::IntercomHandler::new(listener, &user_info, user.event_sender()).run();
                            return polariton::operation::OperationResponse {
                                code: Self::op_code(),
                                return_code: 0,
                                message: polariton::operation::Typed::Null,
                                params: resp_params.into(),
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to start web service intercom listener for user {}: {}", user_info.public_id(), e.error_msg().map(|x| x.to_owned()).unwrap_or("".to_string()));
                            return polariton::operation::OperationResponse {
                                code: Self::op_code(),
                                return_code: oj_rc_core::data::error_codes::WebServicesError::PlatformFeatureNotAvailable as i16,
                                message: polariton::operation::Typed::Null,
                                params: polariton::operation::ParameterTable::with_capacity(0),
                            }
                        },
                    }
                }
            } else {
                log::debug!("Authentication failed for user");
            }
        }
        polariton::operation::OperationResponse {
            code: Self::op_code(),
            return_code: 120,
            message: polariton::operation::Typed::Null,
            params: std::collections::HashMap::new().into(),
        }
    }

    fn handle(&self, _params: polariton::operation::ParameterTable<C>, _user: &Self::User) -> polariton::operation::OperationResponse<C> {
        unreachable!()
    }
}

impl OperationCode for MoreLobbyAuth {
    fn op_code() -> u8 {
        230
    }
}
