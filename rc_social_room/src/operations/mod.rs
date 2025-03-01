mod more_auth;
mod friend_list;
mod settings;
mod clan_invite;
mod clan_info;
mod search_clan;
mod season_rewards;
mod previous_battle_rewards;
mod platoon_data;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .without_state(more_auth::MoreLobbyAuth)
        //.without_state(polariton_server::operations::Ack::<33, _>::default()) // get user clan info (this is equivalent to not being in a clan)
        .without_state(friend_list::friends_provider()) // TODO friend object parsing Token: 0x0200169C RID: 5788
        .without_state(settings::settings_provider()) // TODO save settings persistently
        .without_state(polariton_server::operations::Ack::<43, _>::default()) // get my clan info (this is equivalent to not being in a clan)
        .without_state(clan_invite::clan_invites_provider())
        .without_state(polariton_server::operations::Ack::<19, _>::default()) // get pending platoon invite (this is equivalent to having no pending invite)
        .without_state(clan_info::clan_info_provider())
        .without_state(search_clan::search_clans_provider())
        .without_state(polariton_server::operations::Ack::<52, _>::default()) // validate pending season rewards (this just always needs to be ack-ed)
        .without_state(season_rewards::season_rewards_provider())
        .without_state(previous_battle_rewards::pending_battle_rewards_provider())
        .without_state(platoon_data::platoon_provider())
}
