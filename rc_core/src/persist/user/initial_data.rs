//const REFERENCE_DIR: &str = "layout folder";

fn current_unix_time() -> i64 {
    chrono::Utc::now().timestamp()
}

async fn build_new_account_data(user: &super::UserInfo, db: &rc_database::Database) -> Result<(), rc_database::sea_orm::DbErr> {
    //std::fs::create_dir(&root)?;
    //let garage_dir = root.as_ref().join(super::GARAGE_DIR);
    //std::fs::create_dir(&garage_dir)?;
    let user_data = db.insert_user(default_user_data(user)).await?;
    db.insert_perms(default_user_perms(user_data.id)).await?;
    db.insert_user_aux(default_user_aux_data(user_data.id)).await?;
    db.insert_garages(default_garage_slots(user_data.id)).await?;
    /*for slot in default_garage_slots() {
        let filepath = garage_dir.join(format!("{}.json", slot.slot));
        slot.save(filepath)?;
    }*/
    Ok(())
}

pub async fn setup_new_user(user: &super::UserInfo, db: &rc_database::Database) -> Result<(), rc_database::sea_orm::DbErr> {
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

fn default_user_data(user: &super::UserInfo) -> rc_database::schema::user::ActiveModel {
    let password = if let super::ExtraUserInfo::Standalone { password } = &user.extra {
        //password.to_owned()
        use argon2::password_hash::PasswordHasher;
        let argon2_algo = argon2::Argon2::default();
        let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
        match argon2_algo.hash_password(password.as_bytes(), &salt) {
            Err(e) => {
                log::error!("Failed to hash password for user {}/{}: {}", user.payload.public_id, user.payload.display_name, e);
                "".to_owned()
            },
            Ok(password) => password.to_string(),
        }
    } else {
        Default::default()
    };
    let steam_id = if let super::ExtraUserInfo::Steam { id } = &user.extra {
        Some(id.to_string())
    } else {
        None
    };
    rc_database::schema::user::ActiveModel {
        id: Default::default(),
        creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
        public_id: rc_database::sea_orm::ActiveValue::Set(user.payload.public_id.clone()),
        display_name: rc_database::sea_orm::ActiveValue::Set(user.payload.display_name.clone()),
        password: rc_database::sea_orm::ActiveValue::Set(password),
        email: rc_database::sea_orm::ActiveValue::Set("//TODO".to_owned()),
        steam_id: rc_database::sea_orm::ActiveValue::Set(steam_id),
    }

    /*super::AccountInfo {
        is_mod: false,
        is_admin: false,
        is_dev: false,
        steam_id: None,
        password: None,
        inventory: super::UnlockedParts {
            unlocked: vec![],
            override_: super::inventory::UnlockOverride::UnlockAll,
        },
        garage: super::SelectedGarage {
            uuid: (0, 0),
            slot: 0,
        },
    }*/
}

fn default_user_aux_data(user_id: u32) -> Vec<rc_database::schema::user_aux::ActiveModel> {
    vec![
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::UserXP),
            data: rc_database::sea_orm::ActiveValue::Set("0".to_owned()),
        },
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::PremiumExpiry),
            data: rc_database::sea_orm::ActiveValue::Set(current_unix_time().to_string()),
        },
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::UnlockedParts),
            data: rc_database::sea_orm::ActiveValue::Set(
r#"{
  "unlocked": [],
  "override": "UnlockAll"
}"#.to_owned()),
        },
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::TechPoints),
            data: rc_database::sea_orm::ActiveValue::Set("1337".to_owned()),
        },
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::UserRank),
            data: rc_database::sea_orm::ActiveValue::Set("1".to_owned()),
        },
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::UserFreeCurrency),
            data: rc_database::sea_orm::ActiveValue::Set("10000".to_owned()),
        },
        rc_database::schema::user_aux::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_unix_time()),
            descriptor: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::user_aux::Descriptor::UserPaidCurrency),
            data: rc_database::sea_orm::ActiveValue::Set("1000".to_owned()),
        }
    ]
}

fn default_user_perms(user_id: u32) -> rc_database::schema::permissions::ActiveModel {
    rc_database::schema::permissions::ActiveModel {
        id: Default::default(),
        user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
        moderator: rc_database::sea_orm::ActiveValue::Set(false),
        administrator: rc_database::sea_orm::ActiveValue::Set(false),
        developer: rc_database::sea_orm::ActiveValue::Set(false),
        royalty: rc_database::sea_orm::ActiveValue::Set(false),
        banned: rc_database::sea_orm::ActiveValue::Set(false),
    }
}

fn default_garage_slots(user_id: u32) -> Vec<rc_database::schema::garage::ActiveModel> {
    let current_time = current_unix_time();
    vec![
        rc_database::schema::garage::ActiveModel {
            id: Default::default(),
            user_id: rc_database::sea_orm::ActiveValue::Set(user_id),
            creation_time: rc_database::sea_orm::ActiveValue::Set(current_time),
            slot: rc_database::sea_orm::ActiveValue::Set(0),
            name: rc_database::sea_orm::ActiveValue::Set("Bay 0".to_owned()),
            crf_id: rc_database::sea_orm::ActiveValue::Set(None),
            was_rated: rc_database::sea_orm::ActiveValue::Set(false),
            movement_categories: rc_database::sea_orm::ActiveValue::Set("".to_owned()),
            uuid: rc_database::sea_orm::ActiveValue::Set(current_time),
            thumbnail_version: rc_database::sea_orm::ActiveValue::Set(0),
            total_robot_cpu: rc_database::sea_orm::ActiveValue::Set(0),
            total_cosmetic_cpu: rc_database::sea_orm::ActiveValue::Set(0),
            total_robot_ranking: rc_database::sea_orm::ActiveValue::Set(0),
            bay_cpu: rc_database::sea_orm::ActiveValue::Set(2_000),
            tutorial_robot: rc_database::sea_orm::ActiveValue::Set(false),
            starter_robot_index: rc_database::sea_orm::ActiveValue::Set(None),
            control_type: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::garage::ControlType::Camera),
            vertical_strafing: rc_database::sea_orm::ActiveValue::Set(false),
            sideways_driving: rc_database::sea_orm::ActiveValue::Set(false),
            tracks_turn_on_spot: rc_database::sea_orm::ActiveValue::Set(false),
            mastery_level: rc_database::sea_orm::ActiveValue::Set(1),
            bay_skin_id: rc_database::sea_orm::ActiveValue::Set("RC_MothershipSkin_Neptune_01".to_owned()),
            weapon_order: rc_database::sea_orm::ActiveValue::Set("".to_owned()),
            robot_data: rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
            colour_data: rc_database::sea_orm::ActiveValue::Set(vec![0u8; 4]),
            selected: rc_database::sea_orm::ActiveValue::Set(true),
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
