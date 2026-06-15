#[async_trait::async_trait]
impl super::WebUser for super::account_json::UserData {
    async fn garages(&self) -> Result<Vec<super::GarageWebInfo>, Box<dyn std::error::Error>> {
        let vehicles = self.all_vehicles().await?;
        Ok(vehicles.into_iter()
            .map(|v| super::GarageWebInfo {
                id: v.id,
                slot: v.slot,
                total_robot_cpu: v.total_robot_cpu,
                bay_cpu: v.bay_cpu,
                name: v.name.clone(),
                creation_time: v.creation_time,
            })
            .collect()
        )
    }

    async fn garage_by_id(&self, id: i32) -> Result<Option<super::VehicleData>, Box<dyn std::error::Error>> {
        let garage = self.db.garage_by_id(id).await?;
        Ok(garage.and_then(|g|
            if g.user_id == self.account.id {
                Some(super::VehicleData {
                    name: Some(g.name),
                    slot: g.slot,
                    robot_data: g.robot_data,
                    colour_data: g.colour_data,
                    weapon_order: Vec::default(),
                    crf_id: g.crf_id,
                    was_rated: Some(g.was_rated),
                })
            } else {
                None
            }
        ))
    }

    async fn save_garage(
        &self,
        vehicle: super::VehicleData,
        garage_id: Option<i32>,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_orderer: &crate::cubes::WeaponListParser,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cpu_counts = cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&vehicle.robot_data));
        let weapon_order = weapon_orderer.guess_weapons(&mut std::io::Cursor::new(&vehicle.robot_data));
        let mut entity = oj_rc_database::schema::garage::ActiveModel {
            id: garage_id.map(oj_rc_database::sea_orm::ActiveValue::Set).unwrap_or(oj_rc_database::sea_orm::ActiveValue::NotSet),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
            weapon_order: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::dump_csv(&weapon_order)),
            robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.robot_data),
            colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.colour_data),
            crf_id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            name: if let Some(new_name) = vehicle.name {
                oj_rc_database::sea_orm::ActiveValue::Set(format!("{} (imported)", new_name))
            } else {
                oj_rc_database::sea_orm::ActiveValue::Set(format!(
                    "Import {}",
                    garage_id.map(|x| x.to_string()).unwrap_or_else(|| "(new)".to_owned())
                ))
            },
            total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.total as _),
            total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.cosmetic as _),
            was_rated: oj_rc_database::sea_orm::ActiveValue::Set(false),
            movement_categories: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()), // FIXME
            total_robot_ranking: oj_rc_database::sea_orm::ActiveValue::Set(0), // FIXME
            bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(10_000),
            //tutorial_robot: oj_rc_database::sea_orm::ActiveValue::Set(false),
            mastery_level: oj_rc_database::sea_orm::ActiveValue::Set(0),
            ..super::initial_data::default_reset_slot()
        };
        // TODO charge currency for new or higher CPU bay
        /*let total_cost: u32 = self.garage_upgrades.increments.iter()
            .map(|x| if x.cpu > minimum_upgrade_to_cpu { 0 } else { x.cost })
            .sum();
        log::debug!("Bay CPU upgrade costs {}", total_cost);
        self.currency_sub_checked(super::CurrencyType::Free, total_cost as u64).await.map_err(|e| {
            log::error!("Failed to debit user for cpu upgrade during create of slot {} for user_id {}: {}", slot, self.account.id, e);
            DATABASE_ERR
        })?;*/
        if let Some(garage_id) = garage_id {
            // technically we don't need an account id to get the slot
            // but this way ensures the user also owns the slot we retrieve
            // (and no slot will be retrieved if the user doesn't own the slot, even if the garage id exists)
            if let Some(slot) = self.db.slot_of_garage_by_id_and_user_id(garage_id, self.account.id).await? {
                entity.slot = oj_rc_database::sea_orm::ActiveValue::Set(slot);
                self.db.update_garage_by_user_id_and_slot(entity, self.account.id, slot).await?;
                Ok(())
            } else {
                Err(format!("Invalid vehicle ID {} for user {}", garage_id, self.account.id).into())
            }
        } else {
            let now = chrono::Utc::now().timestamp();
            let new_slot = self.db.garage_max_slot_by_user_id(self.account.id).await? + 1;
            let uuid = super::uuid_sanitize(now);
            entity.slot = oj_rc_database::sea_orm::ActiveValue::Set(new_slot);
            entity.creation_time = oj_rc_database::sea_orm::ActiveValue::Set(now);
            entity.uuid = oj_rc_database::sea_orm::ActiveValue::Set(uuid);
            entity.thumbnail_version = oj_rc_database::sea_orm::ActiveValue::Set(0);
            entity.selected = oj_rc_database::sea_orm::ActiveValue::Set(false);
            self.db.insert_garage(entity).await?;
            Ok(())
        }
    }

    async fn garage_id_selected(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        Ok(self.db.garage_selected(self.account.id).await?
            .map(|x| x.id))
    }
}
