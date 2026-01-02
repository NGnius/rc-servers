//const REFERENCE_DIR: &str = "layout folder";

fn current_unix_time() -> i64 {
    chrono::Utc::now().timestamp()
}

async fn build_new_account_data(user: &super::UserInfo, db: &oj_rc_database::Database) -> Result<(), oj_rc_database::sea_orm::DbErr> {
    let reg_info = super::RegistrationInfo {
        display_name: user.payload.display_name.clone(),
        password: if let super::ExtraUserInfo::Email { password } | super::ExtraUserInfo::Username { password } = &user.extra {
            password.to_owned()
        } else {
            "".to_owned()
        },
        email: None,
        steam_id: if let super::ExtraUserInfo::Steam { id } = &user.extra { Some(*id) } else { None },
    };
    register_new_user(&reg_info, db).await?;
    Ok(())
}

pub async fn setup_new_user(user: &super::UserInfo, db: &oj_rc_database::Database) -> Result<(), oj_rc_database::sea_orm::DbErr> {
    /*let ref_path = new_dir.as_ref().parent().unwrap().join(REFERENCE_DIR);
    if !ref_path.exists() {
        log::debug!("Initialising reference directory {}", ref_path.display());
        build_reference_directory(user)?;
    }
    log::debug!("Copying reference directory for new user: {}", new_dir.as_ref().display());
    so::copy_dir_all(ref_path, new_dir)?;*/
    build_new_account_data(user, db).await?;
    Ok(())
}

pub async fn register_new_user(info: &super::RegistrationInfo, db: &oj_rc_database::Database) -> Result<i32, oj_rc_database::sea_orm::DbErr> {
    let user_data = db.insert_user(default_user_data(info)).await?;
    db.insert_perms(default_user_perms(user_data.id)).await?;
    db.insert_user_aux(default_user_aux_data(user_data.id)).await?;
    db.insert_garages(default_garage_slots(user_data.id)).await?;
    Ok(user_data.id)
}

fn default_user_data(info: &super::RegistrationInfo) -> oj_rc_database::schema::user::ActiveModel {
    let password = {
        use argon2::password_hash::PasswordHasher;
        let argon2_algo = argon2::Argon2::default();
        let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
        match argon2_algo.hash_password(info.password.as_bytes(), &salt) {
            Err(e) => {
                log::error!("Failed to hash password for user {}: {}", info.display_name, e);
                "".to_owned()
            },
            Ok(password) => password.to_string(),
        }
    };
    let steam_id = info.steam_id.map(|id| id.to_string());
    oj_rc_database::schema::user::ActiveModel {
        id: Default::default(),
        creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
        public_id: oj_rc_database::sea_orm::ActiveValue::Set(info.display_name.clone()),
        display_name: oj_rc_database::sea_orm::ActiveValue::Set(info.display_name.clone()),
        password: oj_rc_database::sea_orm::ActiveValue::Set(password),
        email: oj_rc_database::sea_orm::ActiveValue::Set(info.email.clone().unwrap_or_else(|| "".to_owned())),
        steam_id: oj_rc_database::sea_orm::ActiveValue::Set(steam_id),
    }
}

fn default_user_aux_data(user_id: i32) -> Vec<oj_rc_database::schema::user_aux::ActiveModel> {
    let current_time = current_unix_time();
    vec![
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::UserXP),
            data: oj_rc_database::sea_orm::ActiveValue::Set("0".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::PremiumExpiry),
            data: oj_rc_database::sea_orm::ActiveValue::Set(current_time.to_string()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::UnlockedParts),
            data: oj_rc_database::sea_orm::ActiveValue::Set(
r#"{
  "unlocked": [],
  "override": "UnlockAll"
}"#.to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::TechPoints),
            data: oj_rc_database::sea_orm::ActiveValue::Set("1337".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::UserRank),
            data: oj_rc_database::sea_orm::ActiveValue::Set("1".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::UserFreeCurrency),
            data: oj_rc_database::sea_orm::ActiveValue::Set("10000".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::UserPaidCurrency),
            data: oj_rc_database::sea_orm::ActiveValue::Set("1000".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::GarageSlotOrder),
            data: oj_rc_database::sea_orm::ActiveValue::Set("[0]".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels),
            data: oj_rc_database::sea_orm::ActiveValue::Set("[\"sys\"]".to_owned()),
        },
        oj_rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::AvatarId),
            data: oj_rc_database::sea_orm::ActiveValue::Set((current_time % 16).to_string()),
        }
    ]
}

fn default_user_perms(user_id: i32) -> oj_rc_database::schema::permissions::ActiveModel {
    oj_rc_database::schema::permissions::ActiveModel {
        id: Default::default(),
        user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
        moderator: oj_rc_database::sea_orm::ActiveValue::Set(false),
        administrator: oj_rc_database::sea_orm::ActiveValue::Set(false),
        developer: oj_rc_database::sea_orm::ActiveValue::Set(user_id == 1),
        royalty: oj_rc_database::sea_orm::ActiveValue::Set(user_id == 1), // give first user max permissions
        banned: oj_rc_database::sea_orm::ActiveValue::Set(false),
    }
}

pub fn default_new_slot(user_id: i32, slot: i32, bay_cpu: i32) -> oj_rc_database::schema::garage::ActiveModel {
    let current_time = current_unix_time();
    oj_rc_database::schema::garage::ActiveModel {
        id: Default::default(),
        user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
        creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
        slot: oj_rc_database::sea_orm::ActiveValue::Set(slot),
        name: oj_rc_database::sea_orm::ActiveValue::Set(format!("Bay {}", slot)),
        crf_id: oj_rc_database::sea_orm::ActiveValue::Set(None),
        was_rated: oj_rc_database::sea_orm::ActiveValue::Set(false),
        movement_categories: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
        uuid: oj_rc_database::sea_orm::ActiveValue::Set(super::uuid_sanitize(current_time)),
        thumbnail_version: oj_rc_database::sea_orm::ActiveValue::Set(0),
        total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(0),
        total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(0),
        total_robot_ranking: oj_rc_database::sea_orm::ActiveValue::Set(0),
        bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(bay_cpu),
        tutorial_robot: oj_rc_database::sea_orm::ActiveValue::Set(false),
        starter_robot_index: oj_rc_database::sea_orm::ActiveValue::Set(None),
        control_type: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::garage::ControlType::Camera),
        vertical_strafing: oj_rc_database::sea_orm::ActiveValue::Set(false),
        sideways_driving: oj_rc_database::sea_orm::ActiveValue::Set(false),
        tracks_turn_on_spot: oj_rc_database::sea_orm::ActiveValue::Set(false),
        mastery_level: oj_rc_database::sea_orm::ActiveValue::Set(1),
        bay_skin_id: oj_rc_database::sea_orm::ActiveValue::Set("RC_MothershipSkin_Neptune_01".to_owned()),
        death_animation_id: oj_rc_database::sea_orm::ActiveValue::Set("Explosion".to_owned()),
        spawn_animation_id: oj_rc_database::sea_orm::ActiveValue::Set("Spawn".to_owned()),
        weapon_order: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
        robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
        colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
        selected: oj_rc_database::sea_orm::ActiveValue::Set(false),
    }
}

pub fn default_reset_slot() -> oj_rc_database::schema::garage::ActiveModel {
    oj_rc_database::schema::garage::ActiveModel {
        id: Default::default(),
        user_id: Default::default(),
        creation_time: Default::default(),
        slot: Default::default(),
        name: Default::default(),
        crf_id: oj_rc_database::sea_orm::ActiveValue::Set(None),
        was_rated: oj_rc_database::sea_orm::ActiveValue::Set(false),
        movement_categories: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
        uuid: Default::default(),
        thumbnail_version: oj_rc_database::sea_orm::ActiveValue::Set(0),
        total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(0),
        total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(0),
        total_robot_ranking: oj_rc_database::sea_orm::ActiveValue::Set(0),
        bay_cpu: Default::default(),
        tutorial_robot: oj_rc_database::sea_orm::ActiveValue::Set(false),
        starter_robot_index: oj_rc_database::sea_orm::ActiveValue::Set(None),
        control_type: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::garage::ControlType::Camera),
        vertical_strafing: oj_rc_database::sea_orm::ActiveValue::Set(false),
        sideways_driving: oj_rc_database::sea_orm::ActiveValue::Set(false),
        tracks_turn_on_spot: oj_rc_database::sea_orm::ActiveValue::Set(false),
        mastery_level: Default::default(),
        bay_skin_id: oj_rc_database::sea_orm::ActiveValue::Set("RC_MothershipSkin_Neptune_01".to_owned()),
        death_animation_id: oj_rc_database::sea_orm::ActiveValue::Set("Explosion".to_owned()),
        spawn_animation_id: oj_rc_database::sea_orm::ActiveValue::Set("Spawn".to_owned()),
        weapon_order: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
        robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
        colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
        selected: Default::default(),
    }
}

fn default_garage_slots(user_id: i32) -> Vec<oj_rc_database::schema::garage::ActiveModel> {
    let current_time = current_unix_time();
    vec![
        oj_rc_database::schema::garage::ActiveModel {
            id: Default::default(),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(current_time),
            slot: oj_rc_database::sea_orm::ActiveValue::Set(0),
            name: oj_rc_database::sea_orm::ActiveValue::Set("Bay 0".to_owned()),
            crf_id: oj_rc_database::sea_orm::ActiveValue::Set(None),
            was_rated: oj_rc_database::sea_orm::ActiveValue::Set(false),
            movement_categories: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
            uuid: oj_rc_database::sea_orm::ActiveValue::Set(super::uuid_sanitize(current_time)),
            thumbnail_version: oj_rc_database::sea_orm::ActiveValue::Set(0),
            total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(0),
            total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(0),
            total_robot_ranking: oj_rc_database::sea_orm::ActiveValue::Set(0),
            bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(2_000),
            tutorial_robot: oj_rc_database::sea_orm::ActiveValue::Set(false),
            starter_robot_index: oj_rc_database::sea_orm::ActiveValue::Set(None),
            control_type: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::garage::ControlType::Camera),
            vertical_strafing: oj_rc_database::sea_orm::ActiveValue::Set(false),
            sideways_driving: oj_rc_database::sea_orm::ActiveValue::Set(false),
            tracks_turn_on_spot: oj_rc_database::sea_orm::ActiveValue::Set(false),
            mastery_level: oj_rc_database::sea_orm::ActiveValue::Set(1),
            bay_skin_id: oj_rc_database::sea_orm::ActiveValue::Set("RC_MothershipSkin_Neptune_01".to_owned()),
            death_animation_id: oj_rc_database::sea_orm::ActiveValue::Set("Explosion".to_owned()),
            spawn_animation_id: oj_rc_database::sea_orm::ActiveValue::Set("Spawn".to_owned()),
            weapon_order: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
            robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
            colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
            selected: oj_rc_database::sea_orm::ActiveValue::Set(true),
        }
    ]
    /*vec![
        crate::persist::GarageSlot {
            slot: 0,
            name: "Bay 1".to_owned(),
            cubes: 1,
            crf_id: 0,
            was_rated: false,
            movement_categories: vec![crate::persist::ItemCategory::Wheel],
            uuid: (0, 0),
            thumbnail_version: 0,
            total_robot_cpu: 1,
            total_cosmetic_cpu: 0,
            total_robot_ranking: 1,
            bay_cpu: 2_000,
            tutorial_robot: false,
            starter_robot_index: -1,
            control_type: crate::persist::ControlType::Camera,
            control_options: crate::persist::GarageControls { vertical_strafing: false, sideways_driving: false, tracks_turn_on_spot: false, },
            mastery_level: 1,
            bay_skin_id: "RC_MothershipSkin_Neptune_01".to_owned(), // TODO get the rest of the names
            weapon_order: vec![],
            robot_data: vec![0; 4],
            colour_data: vec![0; 4],
        }
    ]*/
}
