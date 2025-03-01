use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::quest::*;

const PARAM_KEY: u8 = 155; // int

pub(super) fn player_daily_quests_provider() -> SimpleFunc<13, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, DailyQuestsInfo {
            can_remove_quest: true,
            player_quests: vec![
                QuestInfo {
                    id: "RE_quest_id0".to_owned(),
                    name: "RE_quest_name0".to_owned(),
                    description: "RE_quest_description0".to_owned(),
                      xp: 1_000,
                      premium_xp: 2_000,
                      robits: 1_000,
                      premium_robits: 1_000,
                      progress_count: 1,
                      target_count: 3,
                      seen: true,
                }
            ],
            completed_quests: Vec::default(),
        }.as_transmissible());
        Ok(params.into())
    })
}
