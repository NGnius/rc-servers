mod eac;
mod more_auth;
mod versioner;
mod maintenancer;
mod game_quality;
mod login_flags;
mod load_analytics;
mod platform_config;
mod tier_banding;
mod cube_list;
mod special_items;
mod premium_config;
mod palette_town;
mod client_config;
mod crf_config;
mod weapon_stats;
mod movement_stats;
mod power_bar_stats;
mod damage_boost_stats;
mod battle_arena_config;
mod cpu_limits_config;
mod cosmetic_config;
mod taunts_config;
mod all_customisations_info;
// some social requests must complete here
mod tech_points;
mod cube_inventory;
mod player_level;
mod balance_info;
mod premium_duration;
mod tutorial_status;
mod user_perms;
mod garage_slots;
mod robopass_season;
mod owned_cosmetics;
mod dev_message;
mod custom_games_maps;
mod avatar_info;
mod custom_game_session;
mod user_xp;
mod garage_upgrades;
mod game_event_params;
mod garage_bay_uuid;
mod tech_tree_data;
mod item_shop_bundles;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .without_state(eac::EacChallengeIgnorer)
        .without_state(more_auth::MoreLobbyAuth)
        .without_state(versioner::VersionTeller)
        .without_state(maintenancer::MaintenanceModeTeller)
        .without_state(game_quality::QualityConfigTeller)
        .without_state(login_flags::UserFlagsTeller)
        .without_state(polariton_server::operations::Ack::<132, _>::default()) // verify user level
        .without_state(load_analytics::NoAnalytics)
        .without_state(polariton_server::operations::Ack::<131, _>::default()) // analytics updated notification
        .without_state(platform_config::platform_config_provider())
        .without_state(tier_banding::tiers_banding_provider())
        .without_state(cube_list::cube_list_provider())
        .without_state(special_items::special_item_list_provider())
        .without_state(premium_config::premium_config_provider())
        .without_state(palette_town::kanto())
        .without_state(client_config::client_config_provider())
        .without_state(crf_config::crf_config_provider())
        .without_state(weapon_stats::weapon_config_provider())
        .without_state(movement_stats::movement_config_provider())
        .without_state(power_bar_stats::power_bar_provider())
        .without_state(damage_boost_stats::damage_boost_provider())
        .without_state(battle_arena_config::battle_arena_config_provider())
        .without_state(cpu_limits_config::cpu_config_provider())
        .without_state(cosmetic_config::cosmetic_limits_config_provider())
        .without_state(taunts_config::taunts_config_provider())
        .without_state(all_customisations_info::all_customisations_provider())
        .without_state(tech_points::tech_points_provider())
        .without_state(cube_inventory::cube_inv_provider())
        .without_state(player_level::player_level_info_provider())
        .without_state(balance_info::balance_wallet_provider())
        .without_state(premium_duration::premium_remaining_provider())
        .without_state(tutorial_status::tutorial_info_provider())
        .without_state(user_perms::user_rights_provider())
        .without_state(garage_slots::garage_slot_provider())
        .without_state(robopass_season::robopass_season_provider())
        .without_state(owned_cosmetics::owned_cosmetics_provider())
        .without_state(owned_cosmetics::selected_cosmetics_provider())
        .without_state(dev_message::dev_message_provider())
        .without_state(custom_games_maps::allowed_maps_provider())
        .without_state(avatar_info::get_avatar_provider())
        .without_state(custom_game_session::get_custom_session_provider())
        .without_state(user_xp::get_user_xp_provider())
        .without_state(garage_upgrades::garage_upgrades_provider())
        .without_state(game_event_params::event_system_params_provider())
        .without_state(garage_bay_uuid::garage_id_provider())
        .without_state(tech_tree_data::tech_tree_layout_provider())
        .without_state(item_shop_bundles::item_bundle_provider())
        //.without_state(polariton_server::operations::Ack::<70, _>::default())
}
