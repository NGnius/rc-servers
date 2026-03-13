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
mod previous_battle_rewards_get;
mod previous_battle_rewards_claim;
mod friend_invite;
mod friend_accept;
mod friend_decline;
mod friend_cancel;
mod friend_remove;
mod platoon_invite_to;
mod user_can_be_platooned;
mod platoon_invite_accept;
mod platoon_invite_decline;
mod platoon_leave;
mod platoon_kick;
mod platoon_invites;
mod platoon_status;
mod clan_create;
mod clan_my_info;
mod clan_join;
mod clan_leave;
mod clan_member_remove;
mod clan_modify;
mod clan_invite_to;
mod clan_invite_accept;
mod clan_invite_decline;
mod clan_invite_decline_all;
mod clan_invite_cancel;
mod clan_member_rerank;
mod clan_experience_poll;
mod user_can_be_custom_gamed;

use polariton_server::operations::OperationsHandler;

pub fn handler(init_ctx: &crate::InitConfig) -> OperationsHandler<crate::UserTy, crate::data::custom::CustomType> {
    OperationsHandler::<crate::UserTy, crate::data::custom::CustomType>::new()
        .modify(oj_rc_core::polariton::RcOpModifier)
        .add(more_auth::more_lobby_auth(init_ctx))
        .add(friend_list::friends_provider(init_ctx)) // TODO friend object parsing Token: 0x0200169C RID: 5788
        .add(settings::settings_provider()) // TODO save settings persistently
        .add(clan_my_info::clan_info_provider())
        .add(clan_invite::clan_invites_provider(init_ctx))
        .add(platoon_invites::platoon_pending_provider(init_ctx))
        .add(clan_info::clan_info_provider(init_ctx))
        .add(search_clan::search_clans_provider())
        .add(polariton_server::operations::Ack::<52, _>::default()) // validate pending season rewards (this just always needs to be ack-ed)
        .add(season_rewards::season_rewards_provider())
        .add(previous_battle_rewards::pending_battle_rewards_provider())
        .add(platoon_data::platoon_provider())
        .add(polariton_server::operations::Ack::<6, _>::default()) // AvatarUpdatedRequest, sent on services_room avatar_set success (just needs to be ack-ed; no params)
        .add(calculate_mmr::mmr_provider())
        .add(polariton_server::operations::Ack::<25, _>::default()) // save social settings, sent on escape menu settings save (should probably be saved someday...)
        .add(friend_invite::friend_invite_provider(init_ctx)) // send friend request, can be sent from match leaderboard
        .add(previous_battle_rewards_get::get_battle_rewards_provider())
        .add(previous_battle_rewards_claim::claim_battle_rewards_provider())
        .add(friend_accept::friend_accept_provider(init_ctx))
        .add(friend_decline::friend_decline_provider(init_ctx))
        .add(friend_cancel::friend_cancel_provider(init_ctx))
        .add(friend_remove::friend_remove_provider(init_ctx))
        .add(platoon_invite_to::platoon_invite_provider(init_ctx))
        .add(user_can_be_platooned::can_invite_to_platoon_provider(init_ctx))
        .add(user_can_be_custom_gamed::can_invite_to_custom_game_provider(init_ctx))
        .add(platoon_invite_accept::platoon_accepter_provider(init_ctx))
        .add(platoon_invite_decline::platoon_decliner_provider(init_ctx))
        .add(platoon_leave::platoon_leave_provider(init_ctx))
        .add(platoon_kick::platoon_kick_provider(init_ctx))
        .add(platoon_status::platoon_update_provider(init_ctx))
        .add(clan_create::creat_clan_provider(init_ctx))
        .add(clan_join::clan_join_provider(init_ctx))
        .add(clan_leave::clan_leave_provider(init_ctx))
        .add(clan_member_remove::clan_remove_provider(init_ctx))
        .add(clan_modify::update_clan_provider(init_ctx))
        .add(clan_invite_to::invite_to_clan_provider(init_ctx))
        .add(clan_invite_accept::clan_accept_provider(init_ctx))
        .add(clan_invite_decline::clan_decline_provider(init_ctx))
        .add(clan_invite_decline_all::clan_decline_all_provider(init_ctx))
        .add(clan_invite_cancel::clan_cancel_provider(init_ctx))
        .add(clan_member_rerank::clan_rank_change_provider(init_ctx))
        .add(clan_experience_poll::clan_experience_provider())
}
