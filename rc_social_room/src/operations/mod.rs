mod more_auth;
mod friend_list;
mod settings;
mod clan_invite;
mod clan_info;
mod search_clan;
mod season_rewards;
mod previous_battle_rewards;
mod platoon_data;
mod calculate_mmr;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy, crate::data::custom::CustomType> {
    OperationsHandler::<crate::UserTy, crate::data::custom::CustomType>::new()
        .modify(oj_rc_core::polariton::RcOpModifier)
        .add(more_auth::MoreLobbyAuth)
        //.add(polariton_server::operations::Ack::<33, _>::default()) // get user clan info (this is equivalent to not being in a clan)
        .add(friend_list::friends_provider()) // TODO friend object parsing Token: 0x0200169C RID: 5788
        .add(settings::settings_provider()) // TODO save settings persistently
        .add(polariton_server::operations::Ack::<43, _>::default()) // get my clan info (this is equivalent to not being in a clan)
        .add(clan_invite::clan_invites_provider())
        .add(polariton_server::operations::Ack::<19, _>::default()) // get pending platoon invite (this is equivalent to having no pending invite)
        .add(clan_info::clan_info_provider())
        .add(search_clan::search_clans_provider())
        .add(polariton_server::operations::Ack::<52, _>::default()) // validate pending season rewards (this just always needs to be ack-ed)
        .add(season_rewards::season_rewards_provider())
        .add(previous_battle_rewards::pending_battle_rewards_provider())
        .add(platoon_data::platoon_provider())
        .add(polariton_server::operations::Ack::<6, _>::default()) // AvatarUpdatedRequest, sent on services_room avatar_set success (just needs to be ack-ed; no params)
        .add(calculate_mmr::mmr_provider())
}
