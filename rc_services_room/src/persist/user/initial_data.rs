const REFERENCE_DIR: &str = "layout folder";

fn build_reference_directory(root: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    std::fs::create_dir(&root)?;
    let garage_dir = root.as_ref().join(super::GARAGE_DIR);
    std::fs::create_dir(&garage_dir)?;
    default_user_data().save(&root)?;
    for slot in default_garage_slots() {
        let filepath = garage_dir.join(format!("{}.json", slot.slot));
        slot.save(filepath)?;
    }
    Ok(())
}

pub fn setup_directory(new_dir: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    let ref_path = new_dir.as_ref().parent().unwrap().join(REFERENCE_DIR);
    if !ref_path.exists() {
        log::debug!("Initialising reference directory {}", ref_path.display());
        build_reference_directory(&ref_path)?;
    }
    log::debug!("Copying reference directory for new user: {}", new_dir.as_ref().display());
    so::copy_dir_all(ref_path, new_dir)?;
    Ok(())
}

fn default_user_data() -> super::AccountInfo {
    super::AccountInfo {
        is_mod: false,
        is_admin: false,
        is_dev: false,
        inventory: super::UnlockedParts {
            unlocked: vec![],
            override_: super::inventory::UnlockOverride::Normal,
        },
        garage: super::SelectedGarage {
            uuid: (0, 0),
            slot: 0,
        },
    }
}

fn default_garage_slots() -> Vec<crate::persist::GarageSlot> {
    vec![
        crate::persist::GarageSlot {
            slot: 0,
            name: "Reverse-engineer great success! slot_name".to_owned(),
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
            weapon_order: vec![0],
            robot_data: vec![0; 4],
            colour_data: vec![0; 4],
        }
    ]
}

mod so {
    // from https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
    use std::path::Path;
    use std::{io, fs};

    pub(super) fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
