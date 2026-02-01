use super::account_json::UserData;

#[async_trait::async_trait]
impl super::FactoryUser for UserData {
    async fn prepare_factory_upload(&self, vehicle: super::VehicleUploadData) -> Result<oj_rc_factory::VehicleUploadInfo, polariton_server::operations::SimpleOpError> {
        self.err_on_banned().await?;
        let slot = self.load_garage_by_slot(vehicle.slot).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle slot {} for user_id {} (prepare_factory_upload): {}", vehicle.slot, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::DatabaseError as i16,
                format!("Failed to retrieve vehicle slot {} while preparing factory upload: {}", vehicle.slot, e),
            )
        })?.ok_or_else(|| {
            log::error!("Failed to find vehicle slot {} for user_id {} (prepare_factory_upload)", vehicle.slot, self.account.id);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::InvalidRobot as i16,
                format!("Failed to find vehicle slot {}", vehicle.slot),
            )
        })?;
        Ok(oj_rc_factory::VehicleUploadInfo {
            name: vehicle.name,
            description: vehicle.description,
            thumbnail: vehicle.thumbnail,
            added_by: self.account.public_id.clone(),
            added_by_display_name: self.account.display_name.clone(),
            added_by_id: self.account.id,
            garage_id: slot.id,
            cpu: slot.total_robot_cpu as u32,
            total_robot_ranking: slot.total_robot_ranking as u32,
            build_version: vehicle.version,
            cube_data: slot.robot_data,
            colour_data: slot.colour_data,
        })
    }

    async fn rate_vehicle(&self, slot: i32, _combat: i32, _cosmetic: i32) -> Result<Option<i32>, polariton_server::operations::SimpleOpError> {
        self.err_on_banned().await?;
        let vehicle = self.load_garage_by_slot(slot).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle slot {} for user_id {} (rate_vehicle): {}", slot, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::DatabaseError as i16,
                format!("Failed to retrieve vehicle slot {} while rating factory upload: {}", slot, e),
            )
        })?.ok_or_else(|| {
            log::error!("Failed to find vehicle slot {} for user_id {} (prepare_factory_upload)", slot, self.account.id);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::InvalidRobot as i16,
                format!("Failed to find vehicle slot {}", slot),
            )
        })?;
        self.db.update_garage(oj_rc_database::schema::garage::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.id),
            was_rated: oj_rc_database::sea_orm::ActiveValue::Set(true),
            ..Default::default()
        }).await
        .map_err(|e| {
            log::error!("Failed to save vehicle slot {} for user_id {} (rate_vehicle): {}", slot, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::DatabaseError as i16,
                format!("Failed to save vehicle slot {} while rating factory upload: {}", slot, e),
            )
        })?;
        Ok(vehicle.crf_id)
    }
}
