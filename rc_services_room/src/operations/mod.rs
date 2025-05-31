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
mod weapon_order;
mod garage_slot_limit;
mod garage_slot_add;
mod garage_slots_order;
mod garage_slot_select;
mod garage_slot_dismantle;
mod garage_slot_upgrade;
mod crf_earnings;
mod crf_list_query;
mod crf_vehicle_data;
mod crf_purchase;
mod crf_upload;
mod avatar_set_custom;
mod avatar_set;
mod garage_slot_controls;
mod garage_slot_set_customisations;
mod garage_slot_name;
mod garage_slot_copy;

use polariton_server::operations::OperationsHandler;

pub fn handler(init_ctx: &crate::InitConfig) -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .modify(rc_core::polariton::OpIdCopy)
        .add(eac::EacChallengeIgnorer)
        .add(more_auth::MoreLobbyAuth)
        .add(versioner::VersionTeller)
        .add(maintenancer::MaintenanceModeTeller)
        .add(game_quality::QualityConfigTeller)
        .add(login_flags::UserFlagsTeller)
        .add(polariton_server::operations::Ack::<132, _>::default()) // verify user level
        .add(load_analytics::NoAnalytics)
        .add(polariton_server::operations::Ack::<131, _>::default()) // analytics updated notification
        .add(platform_config::platform_config_provider())
        .add(tier_banding::tiers_banding_provider())
        .add(cube_list::cube_list_provider(&init_ctx.cubes))
        .add(special_items::special_item_list_provider())
        .add(premium_config::premium_config_provider())
        .add(palette_town::kanto())
        .add(client_config::client_config_provider(&init_ctx.cubes))
        .add(crf_config::crf_config_provider())
        .add(weapon_stats::weapon_config_provider(&init_ctx.cubes))
        .add(movement_stats::movement_config_provider(&init_ctx.cubes))
        .add(power_bar_stats::power_bar_provider())
        .add(damage_boost_stats::damage_boost_provider())
        .add(battle_arena_config::battle_arena_config_provider())
        .add(cpu_limits_config::cpu_config_provider())
        .add(cosmetic_config::cosmetic_limits_config_provider())
        .add(taunts_config::taunts_config_provider())
        .add(all_customisations_info::all_customisations_provider())
        .add(tech_points::tech_points_provider())
        .add(cube_inventory::cube_inv_provider(&init_ctx.cubes))
        .add(player_level::player_level_info_provider())
        .add(balance_info::balance_wallet_provider())
        .add(premium_duration::premium_remaining_provider())
        .add(tutorial_status::tutorial_info_provider())
        .add(user_perms::user_rights_provider())
        .add(garage_slots::garage_slot_provider())
        .add(robopass_season::robopass_season_provider())
        .add(owned_cosmetics::owned_cosmetics_provider())
        .add(owned_cosmetics::selected_cosmetics_provider())
        .add(dev_message::dev_message_provider(&init_ctx.cubes))
        .add(custom_games_maps::allowed_maps_provider())
        .add(avatar_info::avatar_get_provider())
        .add(custom_game_session::get_custom_session_provider())
        .add(user_xp::get_user_xp_provider())
        .add(garage_upgrades::garage_upgrades_provider(&init_ctx.cubes))
        .add(game_event_params::event_system_params_provider(&init_ctx.cubes))
        .add(garage_bay_uuid::garage_id_provider())
        .add(tech_tree_data::tech_tree_layout_provider(&init_ctx.cubes))
        .add(item_shop_bundles::item_bundle_provider())
        .add(robot_customisations::bay_customisations_provider())
        .add(player_data::player_data_provider())
        .add(player_robopass::player_robopass_season_provider())
        .add(weapon_upgrades::weapons_upgrade_provider(&init_ctx.cubes))
        .add(polariton_server::operations::Ack::<172, _>::default()) // custom game change robot tier (param 67 is tier)
        .add(player_rank::rank_provider())
        .add(player_rank::rank_static_provider())
        .add(ab_test_group::test_group_provider())
        .add(league_limits::league_battle_parameters_provider())
        .add(crf_limits::robot_shop_submission_infos_provider())
        .add(robot_mastery_settings::robot_mastery_settings_provider())
        .add(player_started_purchase::started_purchase_provider())
        .add(custom_games_team::team_setup_provider())
        .add(polariton_server::operations::Ack::<152, _>::default()) // custom game player state changed (188 is desired state)
        .add(custom_games_invite::pending_invite_provider())
        .add(chat_settings::chat_settings_provider())
        .add(polariton_server::operations::Ack::<19, _>::default()) // save chat settings
        .add(prebuilt_robots::garage_robot_data_provider())
        .add(prebuilt_colours::garage_colour_combo_provider())
        .add(robopass_preview_items::robopass_preview_provider())
        .add(singleplayer_campaigns::singleplayer_campaigns_provider(&init_ctx.cubes))
        .add(purchases::pending_purchases_provider())
        .add(building_xp_config::building_xp_config_provider())
        .add(weapon_rating_static::weapon_rating_provider())
        .add(weapon_xp_static::weapon_xp_provider())
        .add(machine::garage_machine_provider())
        .add(machine_colour::garage_machine_colour_provider())
        .add(last_completed_campaign::completed_campaign_provider())
        .add(daily_quests::player_daily_quests_provider())
        .add(tech_points::tech_points_awards_provider())
        .add(cube_awards::cube_awards_provider())
        .add(robot_sanction::robot_sanction_provider())
        .add(building_xp::building_xp_save_provider())
        .add(robot_sanction::all_robot_sanctions_provider())
        .add(reconnect_game::available_reconnect_provider())
        .add(machine::garage_machine_save_provider())
        .add(polariton_server::operations::Ack::<32, _>::default()) // TODO handle SaveMachineColorRequest instead of ignoring it
        .add(polariton_server::operations::Ack::<45, _>::default()) // TODO handle UpdateThumbnailVersionRequest instead of ignoring it
        .add(weapon_order::weapon_order_provider(&init_ctx.cubes))
        .add(regen_config::auto_regen_config_provider(&init_ctx.cubes))
        .add(pageantry::after_battle_vote_thresholds_provider(&init_ctx.cubes))
        .add(signup_time::user_signup_date_provider())
        .add(validate_machine::validate_robot_provider())
        .add(game_mode_config::game_mode_config_provider(&init_ctx.cubes))
        .add(score_multipliers_config::tdm_ai_score_config_provider())
        .add(player_robot_rank::player_robot_rank_provider())
        .add(validate_machine::validate_campaign_robot_provider())
        .add(singleplayer_campaigns::singleplayer_complete_campaign_provider(&init_ctx.cubes))
        .add(polariton_server::operations::Ack::<78, _>::default()) // TODO handle SaveCampaignGameAwardsRequest instead of ignoring it
        .add(singleplayer_campaigns::singleplayer_save_complete_campaign_provider()) // TODO handle UpdatePlayerCompletedCampaignWaveRequest saving
        .add(garage_slot_limit::garage_slots_limit(&init_ctx.cubes))
        .add(garage_slot_add::garage_slot_add_provider())
        .add(garage_slots_order::garage_slot_order_provider())
        .add(garage_slot_select::garage_slot_selector())
        .add(garage_slot_dismantle::garage_slot_dismantler())
        .add(garage_slot_upgrade::garage_slot_upgrage_provider())
        .add(crf_earnings::robot_shop_user_earnings_provider())
        .add(crf_list_query::crf_item_list_query_provider(&init_ctx.factory))
        .add(crf_vehicle_data::crf_item_data_provider(&init_ctx.factory))
        .add(crf_purchase::crf_copy_to_bay_provider(&init_ctx.factory, init_ctx.parsers.weapon_order()))
        .add(crf_upload::crf_upload_provider(&init_ctx.factory))
        .add(avatar_set_custom::custom_avatar_upload_handler())
        .add(avatar_set::avatar_set_provider())
        .add(garage_slot_controls::garage_slot_controls_provider())
        .add(garage_slot_set_customisations::garage_slot_customisation_provider())
        .add(garage_slot_name::garage_slot_rename_provider())
        .add(garage_slot_copy::garage_slot_copy_provider())
}
