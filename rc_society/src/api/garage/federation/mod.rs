static SOCIETY_URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub(super) fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    SOCIETY_URL.get_or_init(|| {
        let urls = config.server_config();
        urls.society_url
    });
}

pub fn url(public_id: &str, garage_slot: i32) -> url::Url {
    url::Url::parse(
        &format!("{}/api/v1/activitypub/user/{}/garage/{}", SOCIETY_URL.get().unwrap(), public_id, garage_slot)
    ).unwrap()
}

pub fn build_vehicle_from_garage(garage: oj_rc_core::persist::user::FullVehicleData, public_id: &str) -> oj_serdes::society::activitypub::Vehicle {
    let url_id = url(public_id, garage.slot);
    let now = chrono::Utc::now();
    oj_serdes::society::activitypub::Vehicle {
        kind: oj_serdes::society::activitypub::ActivityPubObjectKind::Vehicle,
        id: url_id.to_string(),
        name: garage.name,
        url: url_id.clone(),
        published: chrono::DateTime::from_timestamp(garage.creation_time, 0).unwrap_or(now),
        updated: Some(now),
        attributed_to: super::super::user::federation::url(public_id),
        cc: Vec::default(),
        slot: garage.slot as _,
        total_robot_cpu: garage.total_robot_cpu as _,
        total_cosmetic_cpu: garage.total_cosmetic_cpu as _,
        total_robot_ranking: garage.total_robot_ranking as _,
        bay_cpu: garage.bay_cpu as _,
        tutorial_robot: false,
        starter_robot_index: None,
        is_camera_controls: matches!(garage.control.control_ty, oj_rc_core::persist::user::ControlType::Camera),
        vertical_strafing: garage.control.vertical_strafing,
        sideways_driving: garage.control.sideways_driving,
        tracks_turn_on_spot: garage.control.tracks_turn_on_spot,
        mastery_level: garage.mastery_level as _,
        bay_skin: garage.bay_skin,
        death_animation: garage.death_animation,
        spawn_animation: garage.spawn_animation,
        weapon_order: garage.weapon_order,
        movement_categories: garage.movement_categories,
        robot_data: garage.robot_data,
        colour_data: garage.colour_data,
        selected: garage.selected,
    }
}
