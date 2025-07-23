use polariton_server::operations::{Operation, OperationCode};
//use polariton::{operation::{Arr, ParameterTable, Typed}, serdes::TypePrefix};

const CODE: u8 = 24;

const MAP_NAMES_PARAM_KEY: u8 = 78;
const VISIBILITY_PARAM_KEY: u8 = 66;
const MODE_PARAM_KEY: u8 = 136;
const AUTO_HEAL_PARAM_KEY: u8 = 37;
const REMAINING_TICKS_PARAM_KEY: u8 = 145;

/* Valid map names
RC_Planet_Mars_02_BA
RC_Planet_Mars_03_BA
RC_Planet_Neptune_02_BA
RC_Planet_Neptune_03_BA
RC_Planet_Earth_01_BA
RC_Planet_Earth_02_BA
RC_Planet_Mars_01_CTF
RC_Planet_Neptune_01_CTF
*/

pub struct GameEventsParamsProvider {
    sequence: std::sync::Mutex<oj_rc_core::persist::config::GameEventSequence>,
}

#[async_trait::async_trait]
impl Operation<()> for GameEventsParamsProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<()>, user: &Self::User) -> polariton::operation::OperationResponse<()> {
        match user.user() {
            Ok(user_info) => {
                let mut params = params.to_dict();
                let current_mode = self.sequence.lock().unwrap().now(user_info.current_game_event_setter());
                params.insert(MAP_NAMES_PARAM_KEY, current_mode.maps);
                params.insert(VISIBILITY_PARAM_KEY, current_mode.visibilities);
                params.insert(MODE_PARAM_KEY, current_mode.modes);
                params.insert(AUTO_HEAL_PARAM_KEY, current_mode.auto_heals);
                params.insert(REMAINING_TICKS_PARAM_KEY, current_mode.remaining_ticks);
                polariton::operation::OperationResponse {
                    code: Self::op_code(),
                    return_code: 0,
                    message: polariton::operation::Typed::Null,
                    params: params.into(),
                }
            },
            Err(e) => {
                polariton::operation::OperationResponse {
                    code: Self::op_code(),
                    return_code: e,
                    message: polariton::operation::Typed::Null,
                    params: params.into(),
                }
            }
        }

    }
}

impl OperationCode for GameEventsParamsProvider {
    fn op_code() -> u8 {
        CODE
    }
}

pub(super) fn event_system_params_provider(conf: &oj_rc_core::ConfigImpl) -> GameEventsParamsProvider {
    let game_seq = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::gamemode_events(conf);
    GameEventsParamsProvider {
        sequence: std::sync::Mutex::new(game_seq),
    }
}
