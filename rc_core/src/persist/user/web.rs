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
                    weapon_order: oj_rc_database::schema::parse_int_csv(&g.weapon_order).into_iter().map(|x| x as i32).collect(),
                    crf_id: g.crf_id,
                    was_rated: Some(g.was_rated),
                })
            } else {
                None
            }
        ))
    }

    async fn garages_full_ordered(&self) -> Result<Vec<super::FullVehicleData>, Box<dyn std::error::Error>> {
        let garages = self.db.garages_by_user_id(self.account.id).await?;
        let order_aux = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::GarageSlotOrder).await?;
        let slot_order = if let Some(order_aux) = order_aux {
            serde_json::from_str(&order_aux.data).unwrap_or_default()
        } else {
            Vec::default()
        };
        let mut transformed_garages = Vec::with_capacity(garages.len());
        for g in garages {
            transformed_garages.push(super::FullVehicleData {
                id: g.id,
                creation_time: g.creation_time,
                slot: g.slot,
                name: g.name,
                crf_id: g.crf_id,
                was_rated: g.was_rated,
                movement_categories: oj_rc_database::schema::parse_int_csv(&g.movement_categories),
                uuid: g.uuid,
                total_robot_cpu: g.total_robot_cpu,
                total_cosmetic_cpu: g.total_cosmetic_cpu,
                total_robot_ranking: g.total_robot_ranking,
                bay_cpu: g.bay_cpu,
                control: super::ControlData {
                    slot: g.slot,
                    control_ty: super::ControlType::from_db(g.control_type),
                    vertical_strafing: g.vertical_strafing,
                    sideways_driving: g.sideways_driving,
                    tracks_turn_on_spot: g.tracks_turn_on_spot,
                },
                mastery_level: g.mastery_level,
                bay_skin: g.bay_skin_id,
                death_animation: g.death_animation_id,
                spawn_animation: g.spawn_animation_id,
                weapon_order: oj_rc_database::schema::parse_int_csv(&g.weapon_order),
                robot_data: g.robot_data,
                colour_data: g.colour_data,
                selected: g.selected,
            });
        }
        transformed_garages.sort_by_key(|g| slot_order.get(g.slot as usize).unwrap_or(&usize::MAX));
        Ok(transformed_garages)
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

    async fn save_federated_garage(&self, is_create: bool, vehicle: super::FullVehicleData, cpu_counter: &crate::cubes::CpuListParser, weapon_orderer: &crate::cubes::WeaponListParser) -> Result<(), Box<dyn std::error::Error>> {
        let cpu_counts = cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&vehicle.robot_data));
        let weapon_order = weapon_orderer.guess_weapons(&mut std::io::Cursor::new(&vehicle.robot_data));
        let real_weapon_order = if weapon_order.len() == vehicle.weapon_order.len() {
            // ensure same weapons; order does not matter
            let mut is_same_weapons = true;
            for weapon in vehicle.weapon_order.iter() {
                if !weapon_order.contains(&(*weapon as i32)) {
                    is_same_weapons = false;
                    break;
                }
            }
            if is_same_weapons {
                oj_rc_database::schema::dump_csv(&vehicle.weapon_order)
            } else {
                oj_rc_database::schema::dump_csv(&weapon_order)
            }
        } else {
            oj_rc_database::schema::dump_csv(&weapon_order)
        };
        let garage_opt = self.db.garage_by_user_id_and_slot(self.account.id, vehicle.slot).await?;
        if let Some(garage) = garage_opt {
            if is_create {
                log::warn!("Got federated create vehicle for existing garage slot of user {}, slot {} (updating instead)", self.account.public_id, vehicle.slot);
            }
            // update garage
            let update = oj_rc_database::schema::garage::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::Set(garage.id),
                user_id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                creation_time: oj_rc_database::sea_orm::ActiveValue::NotSet,
                slot: oj_rc_database::sea_orm::ActiveValue::NotSet,
                name: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.name),
                crf_id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                was_rated: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.was_rated),
                movement_categories: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::dump_csv(&vehicle.movement_categories)),
                uuid: oj_rc_database::sea_orm::ActiveValue::NotSet,
                thumbnail_version: oj_rc_database::sea_orm::ActiveValue::Set(garage.thumbnail_version + 1),
                total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.total as _),
                total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.cosmetic as _),
                total_robot_ranking: oj_rc_database::sea_orm::ActiveValue::NotSet,
                bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.bay_cpu),
                tutorial_robot: oj_rc_database::sea_orm::ActiveValue::NotSet,
                starter_robot_index: oj_rc_database::sea_orm::ActiveValue::NotSet,
                control_type: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.control_ty.into_db()),
                vertical_strafing: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.vertical_strafing),
                sideways_driving: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.sideways_driving),
                tracks_turn_on_spot: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.tracks_turn_on_spot),
                mastery_level: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.mastery_level),
                bay_skin_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.bay_skin),
                death_animation_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.death_animation),
                spawn_animation_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.spawn_animation),
                weapon_order: oj_rc_database::sea_orm::ActiveValue::Set(real_weapon_order),
                robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.robot_data),
                colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.colour_data),
                selected: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.selected),
            };
            self.db.update_garage(update).await?;
        } else {
            if is_create {
                log::warn!("Got federated update vehicle for non-existent garage slot of user {}, slot {} (creating instead)", self.account.public_id, vehicle.slot);
            }
            // create garage
            let now = chrono::Utc::now().timestamp();
            // don't let misconfigured remote instances put strange values into our database
            let (creation_time, uuid) = if vehicle.creation_time >= now {
                (now, super::uuid_sanitize(now ^ vehicle.uuid))
            } else {
                (vehicle.creation_time, vehicle.uuid)
            };
            let insert = oj_rc_database::schema::garage::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(creation_time),
                slot: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.slot),
                name: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.name),
                crf_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.crf_id),
                was_rated: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.was_rated),
                movement_categories: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::dump_csv(&vehicle.movement_categories)),
                uuid: oj_rc_database::sea_orm::ActiveValue::Set(uuid),
                thumbnail_version: oj_rc_database::sea_orm::ActiveValue::Set(1),
                total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.total as _),
                total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.cosmetic as _),
                total_robot_ranking: oj_rc_database::sea_orm::ActiveValue::Set(0),
                bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.bay_cpu),
                tutorial_robot: oj_rc_database::sea_orm::ActiveValue::Set(false),
                starter_robot_index: oj_rc_database::sea_orm::ActiveValue::Set(None),
                control_type: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.control_ty.into_db()),
                vertical_strafing: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.vertical_strafing),
                sideways_driving: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.sideways_driving),
                tracks_turn_on_spot: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.control.tracks_turn_on_spot),
                mastery_level: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.mastery_level),
                bay_skin_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.bay_skin),
                death_animation_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.death_animation),
                spawn_animation_id: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.spawn_animation),
                weapon_order: oj_rc_database::sea_orm::ActiveValue::Set(real_weapon_order),
                robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.robot_data),
                colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.colour_data),
                selected: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.selected),
            };
            self.db.insert_garage(insert).await?;
        }
        Ok(())
    }

    async fn garage_id_selected(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        Ok(self.db.garage_selected(self.account.id).await?
            .map(|x| x.id))
    }

    async fn garage_stats(&self) -> Result<super::GarageWebStats, Box<dyn std::error::Error>> {
        let vehicle_count = self.db.garage_count_by_user_id(self.account.id).await?;
        let regular_count = self.db.garage_count_by_user_id_between(self.account.id, 2_000, 1).await?;
        let mega_count = self.db.garage_count_by_user_id_between(self.account.id, u32::MAX, 2_000).await?;
        let empty_count = self.db.garage_count_by_user_id_between(self.account.id, 0, 0).await?;
        let factory_count = self.db.garage_count_by_user_id_factory(self.account.id).await?;
        let storage_size = self.db.garage_storage_by_user_id(self.account.id).await?;
        let selected_garage_slot = self.db.garage_selected(self.account.id).await?.map(|g| g.slot).unwrap_or_default();
        Ok(super::GarageWebStats {
            vehicle_total: vehicle_count,
            regular_vehicle_total: regular_count,
            mega_vehicle_total: mega_count,
            empty_vehicle_total: empty_count,
            factory_vehicle_total: factory_count,
            storage_bytes_total: storage_size.map(|size| size.try_into().unwrap_or(0)),
            selected_garage: selected_garage_slot,
        })
    }

    async fn account_stats(&self) -> Result<super::AccountWebStats, Box<dyn std::error::Error>> {
        let user_aux_data = self.db.user_aux_by_user_id(self.account.id).await?;
        let mut currency_map = std::collections::HashMap::with_capacity(8);
        let mut avatar_id = None;
        let mut premium_until = 0;
        let mut user_rank = 0;
        let mut last_seen = self.account.creation_time;
        for row in user_aux_data.iter() {
            match row.descriptor {
                oj_rc_database::schema::user_aux::Descriptor::UserXP => {
                    currency_map.insert(super::CurrencyType::Experience, row.data.parse().unwrap_or_default());
                },
                oj_rc_database::schema::user_aux::Descriptor::TechPoints => {
                    currency_map.insert(super::CurrencyType::TechPoints, row.data.parse().unwrap_or_default());
                },
                oj_rc_database::schema::user_aux::Descriptor::UserFreeCurrency => {
                    currency_map.insert(super::CurrencyType::Free, row.data.parse().unwrap_or_default());
                },
                oj_rc_database::schema::user_aux::Descriptor::UserPaidCurrency => {
                    currency_map.insert(super::CurrencyType::Paid, row.data.parse().unwrap_or_default());
                },
                oj_rc_database::schema::user_aux::Descriptor::AvatarId => {
                    avatar_id = row.data.parse::<i32>().ok();
                },
                oj_rc_database::schema::user_aux::Descriptor::PremiumExpiry => {
                    premium_until = row.data.parse().unwrap_or_default();
                },
                oj_rc_database::schema::user_aux::Descriptor::UserRank => {
                    user_rank = row.data.parse().unwrap_or_default();
                },
                oj_rc_database::schema::user_aux::Descriptor::LastSeen => {
                    last_seen = row.data.parse().unwrap_or_default();
                },
                _ => {},
            }
        }
        let game_count = self.db.count_games_by_user_id(self.account.id).await?;
        Ok(super::AccountWebStats {
            currencies: currency_map,
            games_played: game_count,
            avatar_id,
            premium_until,
            rank: user_rank,
            creation_time: self.account.creation_time,
            last_seen_time: last_seen,
        })
    }

    async fn sanction_stats(&self) -> Result<super::SanctionWebStats, Box<dyn std::error::Error>> {
        let all_count = self.db.count_sanctions_by_user_id(self.account.id).await?;
        let pending_count = self.db.count_sanctions_by_user_id_and_ack(self.account.id, false).await?;
        let warn_count = self.db.count_sanctions_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::sanction::Descriptor::Warn).await?;
        let mute_count = self.db.count_sanctions_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::sanction::Descriptor::Mute).await?;
        let first_active_mute = self.db.sanction_by_user_id_and_descriptor_and_active(self.account.id, oj_rc_database::schema::sanction::Descriptor::Mute).await?;
        Ok(super::SanctionWebStats {
            total: all_count,
            pending_total: pending_count,
            warn_total: warn_count,
            mute_total: mute_count,
            muted_until: first_active_mute.map(|s| s.creation_time + s.duration.unwrap_or_default()),
        })
    }

    async fn social_stats(&self) -> Result<super::SocialWebStats, Box<dyn std::error::Error>> {
        let clan_opt = self.db.clan_by_user_id(self.account.id).await?
            .map(|(clan, _myself)| super::ClanData {
                name: clan.name,
                description: clan.description,
                ty: super::ClanType::db_to_core(&clan.variant),
                size: 0,
            });
        let friends_count = self.db.count_friends_by_user_id(self.account.id, oj_rc_database::schema::friend::FriendStatus::Accepted).await?;
        let friends_of_count = self.db.count_friends_target_by_user_id(self.account.id, oj_rc_database::schema::friend::FriendStatus::Accepted).await?;
        let subbed_chats_aux = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await?;
        let chat_list: Vec<String> = subbed_chats_aux.and_then(|aux| serde_json::from_str(&aux.data).ok()).unwrap_or_default();
        Ok(super::SocialWebStats {
            clan: clan_opt,
            friends_total: friends_count,
            friends_of_total: friends_of_count,
            chats: chat_list,
        })
    }

    async fn fedi_info(&self) -> Result<super::FederationWebData, Box<dyn std::error::Error>> {
        let servers = self.db.federations_all().await?;
        Ok(super::FederationWebData {
            federations: servers.into_iter()
                .map(|server| super::FederationWebDetails {
                    id: server.id,
                    domain: server.domain,
                    auth: server.auth,
                    society: server.society,
                    last_used: server.last_used_time,
                    first_used: server.creation_time,
                })
                .collect()
        })
    }

    /// Delete own account in the database, dangerous!
    async fn delete_account(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rows_deleted = self.db.delete_account(self.account.id).await?;
        log::info!("Deleted account {} data, {} rows", self.account.id, rows_deleted);
        Ok(())
    }

    async fn set_display_name(&self, new_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = self.db.user_by_any_unique_id(new_name.to_owned()).await? {
            if user.id != self.account.id {
                return Err(Box::from(format!("Name {} is already in use", new_name)));
            }
        }
        self.db.update_user(oj_rc_database::schema::user::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
            display_name: oj_rc_database::sea_orm::ActiveValue::Set(new_name.to_owned()),
            ..Default::default()
        }).await?;
        log::info!("Renamed account {} display from {} to {}", self.account.id, self.account.display_name, new_name);
        Ok(())
    }
}
