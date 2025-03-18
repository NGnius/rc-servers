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
mod robot_customisations;
mod player_data;
mod player_robopass;
mod weapon_upgrades;
mod player_rank;
mod ab_test_group;
mod league_limits;
mod crf_limits;
mod robot_mastery_settings;
mod player_started_purchase;
mod custom_games_team;
mod custom_games_invite;
mod chat_settings;
mod prebuilt_robots;
mod prebuilt_colours;
mod robopass_preview_items;
mod singleplayer_campaigns;
mod purchases;
mod building_xp_config;
mod weapon_rating_static;
mod weapon_xp_static;
mod machine;
mod machine_colour;
mod last_completed_campaign;
mod daily_quests;
mod cube_awards;
mod robot_sanction;
mod building_xp;
mod reconnect_game;
mod regen_config;
mod pageantry;
mod signup_time;
mod validate_machine;
mod game_mode_config;
mod score_multipliers_config;
mod player_robot_rank;

use polariton_server::operations::OperationsHandler;

pub fn handler(init_ctx: &crate::InitConfig) -> OperationsHandler<crate::UserTy> {
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
        .without_state(cube_list::cube_list_provider(&init_ctx.cubes))
        .without_state(special_items::special_item_list_provider())
        .without_state(premium_config::premium_config_provider())
        .without_state(palette_town::kanto())
        .without_state(client_config::client_config_provider())
        .without_state(crf_config::crf_config_provider())
        .without_state(weapon_stats::weapon_config_provider(&init_ctx.cubes))
        .without_state(movement_stats::movement_config_provider(&init_ctx.cubes))
        .without_state(power_bar_stats::power_bar_provider())
        .without_state(damage_boost_stats::damage_boost_provider())
        .without_state(battle_arena_config::battle_arena_config_provider())
        .without_state(cpu_limits_config::cpu_config_provider())
        .without_state(cosmetic_config::cosmetic_limits_config_provider())
        .without_state(taunts_config::taunts_config_provider())
        .without_state(all_customisations_info::all_customisations_provider())
        .without_state(tech_points::tech_points_provider())
        .without_state(cube_inventory::cube_inv_provider(&init_ctx.cubes))
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
        .without_state(tech_tree_data::tech_tree_layout_provider(&init_ctx.cubes))
        .without_state(item_shop_bundles::item_bundle_provider())
        .without_state(robot_customisations::bay_customisations_provider())
        .without_state(player_data::player_data_provider())
        .without_state(player_robopass::player_robopass_season_provider())
        .without_state(weapon_upgrades::weapons_upgrade_provider(&init_ctx.cubes))
        .without_state(polariton_server::operations::Ack::<172, _>::default()) // custom game change robot tier (param 67 is tier)
        .without_state(player_rank::rank_provider())
        .without_state(player_rank::rank_static_provider())
        .without_state(ab_test_group::test_group_provider())
        .without_state(league_limits::league_battle_parameters_provider())
        .without_state(crf_limits::robot_shop_submission_infos_provider())
        .without_state(robot_mastery_settings::robot_mastery_settings_provider())
        .without_state(player_started_purchase::started_purchase_provider())
        .without_state(custom_games_team::team_setup_provider())
        .without_state(polariton_server::operations::Ack::<152, _>::default()) // custom game player state changed (188 is desired state)
        .without_state(custom_games_invite::pending_invite_provider())
        .without_state(chat_settings::chat_settings_provider())
        .without_state(prebuilt_robots::garage_robot_data_provider())
        .without_state(prebuilt_colours::garage_colour_combo_provider())
        .without_state(robopass_preview_items::robopass_preview_provider())
        .without_state(singleplayer_campaigns::singleplayer_campaigns_provider())
        .without_state(purchases::pending_purchases_provider())
        .without_state(building_xp_config::building_xp_config_provider())
        .without_state(weapon_rating_static::weapon_rating_provider())
        .without_state(weapon_xp_static::weapon_xp_provider())
        .without_state(machine::garage_machine_provider())
        .without_state(machine_colour::garage_machine_colour_provider())
        .without_state(last_completed_campaign::completed_campaign_provider())
        .without_state(daily_quests::player_daily_quests_provider())
        .without_state(tech_points::tech_points_awards_provider())
        .without_state(cube_awards::cube_awards_provider())
        .without_state(robot_sanction::robot_sanction_provider())
        .without_state(building_xp::building_xp_save_provider())
        .without_state(robot_sanction::all_robot_sanctions_provider())
        .without_state(reconnect_game::available_reconnect_provider())
        .without_state(machine::garage_machine_save_provider())
        .without_state(polariton_server::operations::Ack::<32, _>::default()) // TODO handle SaveMachineColorRequest instead of ignoring it
        .without_state(polariton_server::operations::Ack::<45, _>::default()) // TODO handle UpdateThumbnailVersionRequest instead of ignoring it
        .without_state(machine::weapon_order_provider())
        .without_state(regen_config::auto_regen_config_provider(&init_ctx.cubes))
        .without_state(pageantry::after_battle_vote_thresholds_provider(&init_ctx.cubes))
        .without_state(signup_time::user_signup_date_provider())
        .without_state(validate_machine::validate_robot_provider())
        .without_state(game_mode_config::game_mode_config_provider(&init_ctx.cubes))
        .without_state(score_multipliers_config::tdm_ai_score_config_provider())
        .without_state(player_robot_rank::player_robot_rank_provider())
}
