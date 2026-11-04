use super::account_json::UserData;

#[async_trait::async_trait]
impl super::ServiceUser for UserData {
    async fn all_slots(&self) -> super::UserSlots {
        let slots = match self.all_vehicles().await {
            Ok(slots) => slots,
            Err(e) => {
                log::error!("Failed to load all vehicles: {}", e);
                Vec::default()
            }
        };
        let slot_order = match self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::GarageSlotOrder).await{
            Ok(Some(slot_order_db)) => {
                let slots = serde_json::from_str::<Vec<u32>>(&slot_order_db.data).unwrap_or_default();
                polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(*slot as _)).collect::<Vec<_>>().into())
            },
            Ok(None) => {
                log::error!("No vehicle slot order for user_id {}", self.account.id);
                polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into())
            },
            Err(e) => {
                log::error!("Failed to get vehicle slot order for user_id {}: {}", self.account.id, e);
                polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into())
            },
        };
        //let slot_order = polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into());
        let slot_info = polariton::operation::Typed::Dict(polariton::operation::Dict {
            key_ty: polariton::serdes::TypePrefix::Int,
            val_ty: polariton::serdes:: TypePrefix::HashMap,
            items: slots.into_iter().map(|slot| {
                let slot_index = slot.slot;
                let garage_data: crate::data::garage_bay::GarageSlotInfo = crate::persist::garage::db_into_data(slot);
                (polariton::operation::Typed::Int(slot_index as _), garage_data.as_transmissible())
            }).collect(),
        });
        super::UserSlots {
            slot_info, slot_order,
        }
    }

    async fn slot_by_id(&self, id: i32) -> Result<crate::persist::user::UserSlotData, polariton_server::operations::SimpleOpError> {
        self.err_on_banned().await?;
        match self.load_garage_by_slot(id as _).await {
            Ok(Some(slot)) => {
                let cube_count = slot.cube_count() as i32;
                let control_ty: crate::data::garage_bay::ControlType = crate::persist::garage::control_ty_into_data(slot.control_type);
                let control_options = crate::data::garage_bay::ControlOptions {
                    vertical_strafing: slot.vertical_strafing,
                    sideways_driving: slot.sideways_driving,
                    tracks_turn_on_spot: slot.tracks_turn_on_spot,
                };
                Ok(crate::persist::user::UserSlotData {
                    data: polariton::operation::Typed::Bytes(slot.robot_data.into()),
                    colour_data: polariton::operation::Typed::Bytes(slot.colour_data.into()),
                    cube_count: polariton::operation::Typed::Int(cube_count),
                    weapon_order: polariton::operation::Typed::IntArr(oj_rc_database::schema::parse_int_csv(&slot.weapon_order).into_iter().map(|x| x as i32).collect::<Vec<_>>().into()),
                    movement_categories: polariton::operation::Typed::IntArr(oj_rc_database::schema::parse_int_csv(&slot.movement_categories).into_iter().map(|x| x as i32).collect::<Vec<_>>().into()),
                    control_type: polariton::operation::Typed::Int(control_ty as _),
                    control_options: control_options.as_transmissible(),
                    mastery_level: polariton::operation::Typed::Int(slot.mastery_level),
                    robot_rank: polariton::operation::Typed::Int(slot.total_robot_ranking as _),
                    cpu: polariton::operation::Typed::Int(slot.total_robot_cpu as _),
                    cosmetic_cpu: polariton::operation::Typed::Int(slot.total_cosmetic_cpu as _),
                    uuid: polariton::operation::Typed::Str(super::i64_as_uuid_str(slot.uuid).into()),
                })
            },
            Ok(None) => {
                log::error!("Failed to find vehicle slot {} for user_id {}", id, self.account.id);
                Err(polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16))
            }
            Err(e) => {
                log::error!("Failed to retrieve vehicle {} for user_id {}: {}", id, self.account.id, e);
                Err(polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::InvalidRobot as i16))
            }
        }
    }

    async fn new_slot(&self, reset_slot: Option<i32>) -> Result<super::NewSlotData, polariton_server::operations::SimpleOpError> {
        self.err_on_banned().await?;
        let model = if let Some(slot) = reset_slot {
            let new_data = super::initial_data::default_reset_slot();
            if let Some(reset_g) = self.db.update_garage_by_user_id_and_slot(new_data, self.account.id, slot).await.map_err(|e| {
                log::error!("Failed to reset vehicle slot {} for user_id {}: {}", slot, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })? {
                reset_g
            } else {
                log::warn!("No vehicle slot {} to reset for user_id {}, creating new slot", slot, self.account.id);
                return self.new_slot(None).await;
            }
        } else {
            let max_slot = self.db.garage_max_slot_by_user_id(self.account.id).await.map_err(|e| {
                log::error!("Failed to get max slot for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })?;
            let next_slot = max_slot + 1;
            let new_data = super::initial_data::default_new_slot(self.account.id, next_slot, 2_000);
            self.db.insert_garage(new_data).await.map_err(|e| {
                log::error!("Failed to create new vehicle slot {} for user_id {}: {}", next_slot, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })?
        };
        self.db.update_garage_selected_by_user_id_and_slot(self.account.id, model.slot).await
            .map_err(|e| {
                log::error!("Failed to select new vehicle slot {} for user_id {}: {}", model.slot, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })?;
        let split_uuid = super::i64_split(model.uuid);
        Ok(super::NewSlotData {
            name: polariton::operation::Typed::Str(model.name.into()),
            uuid_0: polariton::operation::Typed::Str(split_uuid.0.to_string().into()), // yes, seriously
            uuid_1: polariton::operation::Typed::Str(split_uuid.1.to_string().into()), // also yes, seriously
            slot: polariton::operation::Typed::Int(model.slot as _),
            bay_cpu: polariton::operation::Typed::Int(model.bay_cpu as _),
            mastery_level: polariton::operation::Typed::Int(model.mastery_level as _),
            slot_i: model.slot as _,
        })
    }

    async fn upgrade_slot(&self, increments: i32) -> Result<polariton::operation::Typed, polariton_server::operations::SimpleOpError> {
        self.err_on_banned().await?;
        if increments <= 0 {
            // no-op
            return Ok(polariton::operation::Typed::Bool(true));
        }
        let selected_slot = self.db.garage_selected(self.account.id).await.map_err(|e| {
            log::error!("Failed to retrieve selected vehicle slot for user_id {}: {}", self.account.id, e);
            polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
        })?.ok_or_else(|| {
            log::error!("No selected vehicle slot for user_id {}", self.account.id);
            polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
        })?;
        let inc_opt = self.garage_upgrades.increments.iter().enumerate().rfind(|(_i, inc)| inc.cpu <= selected_slot.bay_cpu as u32);
        if let Some((i, _)) = inc_opt {
            let max_upgrade = self.garage_upgrades.increments.len() - 1;
            let upgrade_to = i + (increments as usize);
            if upgrade_to > max_upgrade {
                // over-upgraded
                Ok(polariton::operation::Typed::Bool(false))
            } else {
                let upgrade_to_cpu = self.garage_upgrades.increments[upgrade_to].cpu;
                // subtract bay cpu upgrade cost from user free currency
                let total_cost: u32 = self.garage_upgrades.increments[i+1..=upgrade_to].iter().map(|x| x.cost).sum();
                self.currency_sub_checked(super::CurrencyType::Free, total_cost as u64).await.map_err(|e| {
                    log::error!("Failed to debit user for cpu upgrade of {} to selected vehicle slot for user_id {}: {}", upgrade_to_cpu, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
                })?;
                // apply upgrade to bay
                let entity = oj_rc_database::schema::garage::ActiveModel {
                    bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(upgrade_to_cpu as i32),
                    ..Default::default()
                };
                self.db.update_garage_by_user_id_and_slot(entity, self.account.id, selected_slot.slot).await.map_err(|e| {
                    log::error!("Failed to upgrade selected vehicle slot to bay cpu of {} for user_id {}: {}", upgrade_to_cpu, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
                })?;
                Ok(polariton::operation::Typed::Bool(true))
            }
        } else {
            // probably a bad/changed garage update config
            log::warn!("No vehicle slot upgrade found for bay CPU {} for user_id {}", selected_slot.bay_cpu, self.account.id);
            Ok(polariton::operation::Typed::Bool(false))
        }
    }

    async fn get_slot_customisations(&self, uuid: &str) -> Result<super::GetCustomisationData, polariton_server::operations::SimpleOpError> {
        if let Some(uuid) = super::str_to_i64(uuid) {
            let garage_opt = self.db.garage_by_uuid(uuid).await.map_err(|e| {
                log::error!("Failed to retrieve garage {} for user_id {}: {}", uuid, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })?;
            if let Some(garage) = garage_opt {
                Ok(super::GetCustomisationData {
                    bay: polariton::operation::Typed::Str(garage.bay_skin_id.into()),
                    spawn: polariton::operation::Typed::Str(garage.spawn_animation_id.into()),
                    death: polariton::operation::Typed::Str(garage.death_animation_id.into()),
                })
            } else {
                Err(polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16))
            }
        } else {
            Err(polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::UnexpectedError as i16))
        }
    }

    async fn get_avatar_info(&self) -> Result<super::GetAvatarInfo, polariton_server::operations::SimpleOpError> {
        self.err_on_banned().await?;
        let avatar_id_aux = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve AvatarId (user_aux) for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })?
            .ok_or_else(|| {
                log::error!("Failed to find AvatarId (user_aux) for user_id {}", self.account.id);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            })?;
        let avatar_id: u32 = avatar_id_aux.data.parse()
            .map_err(|e| {
                log::error!("Failed to parse AvatarId (user_aux) for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::UnexpectedError as i16)
            })?;
        Ok(super::GetAvatarInfo {
            avatar_id: polariton::operation::Typed::Int(if avatar_id == u32::MAX { 0 } else { avatar_id as i32 }),
            use_custom: polariton::operation::Typed::Bool(avatar_id == u32::MAX),
        })
    }
}
