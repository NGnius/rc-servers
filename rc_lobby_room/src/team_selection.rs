pub struct InitedTeamChoosers {
    pub battle_arena: oj_rc_core::persist::user::StandardTeamChooser,
    pub elimination: oj_rc_core::persist::user::StandardTeamChooser,
    pub pit: oj_rc_core::persist::user::StandardTeamChooser,
    pub team_deathmatch: oj_rc_core::persist::user::StandardTeamChooser,
}

pub fn choosers_from_conf(conf: &oj_rc_core::persist::config::TeamChoosers, plugins_path: impl AsRef<std::path::Path>) -> InitedTeamChoosers {
    InitedTeamChoosers {
        battle_arena: chooser_from_conf(&conf.battle_arena, &plugins_path),
        elimination: chooser_from_conf(&conf.elimination, &plugins_path),
        pit: chooser_from_conf(&conf.pit, &plugins_path),
        team_deathmatch: chooser_from_conf(&conf.team_deathmatch, &plugins_path),
    }
}

fn chooser_from_conf(conf: &oj_rc_core::persist::TeamChooser, plugins_path: impl AsRef<std::path::Path>) -> oj_rc_core::persist::user::StandardTeamChooser {
    match conf {
        oj_rc_core::persist::TeamChooser::Alternating => oj_rc_core::persist::user::StandardTeamChooser::alternating(),
        oj_rc_core::persist::TeamChooser::AllOnOne { team } => oj_rc_core::persist::user::StandardTeamChooser::AllOn(*team),
        oj_rc_core::persist::TeamChooser::OneOnAll => oj_rc_core::persist::user::StandardTeamChooser::OnePer,
        oj_rc_core::persist::TeamChooser::Custom { path } => {
            let full_path = plugins_path.as_ref().join(path);
            log::warn!("Custom team selector plugin {} is experimental and insecure", full_path.display());
            let result = oj_rc_plugins::team_selection::TeamSelectorCPlugin::new(&full_path);
            match result {
                Ok(c_plugin) => oj_rc_core::persist::user::StandardTeamChooser::Custom(Box::new(TeamSelectionPluginWrapper(c_plugin)) as _),
                Err(e) => {
                    log::error!("Failed to load custom team selector plugin {}: {} (crashing!)", full_path.display(), e);
                    panic!("Failed to load custom team selector plugin {}: {}", full_path.display(), e)
                }
            }
        }
    }
}

struct TeamSelectionPluginWrapper<T: oj_rc_plugins::team_selection::TeamSelector>(T);

impl <T: oj_rc_plugins::team_selection::TeamSelector> oj_rc_core::persist::user::TeamChooser for TeamSelectionPluginWrapper<T> {
    fn choose_team(&self, game: &str, index: usize, player: &oj_rc_core::persist::user::PlayerLobbyDescriptor) -> i32 {
        self.0.select_team(
            game,
            index,
            if player.user_id >= 0 { Some(player.user_id) } else { None },
            player.group.clone(),
        ) as i32
    }
}
