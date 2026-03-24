//! The foreign function interface implementation for assigning a team to a player entering a match in different shared objects/libraries.
use std::ffi::{CString, c_char};

const SELECT_TEAM_SYMBOL_NAME: &[u8] = b"oj_rc_select_team";
const SELECT_TEAM_SYMBOL_NAME_STR: &str = "oj_rc_select_team";

pub struct TeamSelectorCPlugin {
    dll: libloading::Library,
    pretty_name: String,
}

impl TeamSelectorCPlugin {
    pub fn new(file: impl AsRef<std::path::Path>) -> Result<Self, libloading::Error> {
        let dll = unsafe { libloading::Library::new(file.as_ref()) }?;
        Ok(Self {
            dll,
            pretty_name: file.as_ref().display().to_string(),
        })
    }
}

impl super::TeamSelector for TeamSelectorCPlugin {
    fn select_team(&self, game: &str, index: usize, user_id: Option<i32>, group: Option<String>) -> u8 {
        let func: libloading::Symbol<unsafe extern "C" fn(*const c_char, u64, *const i32, *const c_char) -> u8> = match unsafe { self.dll.get(SELECT_TEAM_SYMBOL_NAME) } {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to find symbol {} in library {}: {}", SELECT_TEAM_SYMBOL_NAME_STR, self.pretty_name, e);
                return 0;
            }
        };
        let game_c = CString::new(game).unwrap_or_default();
        let index_c = index as u64;
        let user_id_c = if let Some(user_id) = &user_id {
            std::ptr::from_ref(user_id)
        } else {
            std::ptr::null()
        };
        let group_c = group.map(|group| CString::new(group).unwrap_or_default());
        unsafe {
            func(
                game_c.as_ptr(),
                index_c,
                user_id_c,
                group_c.map(|x| x.as_ptr())
                    .unwrap_or(std::ptr::null()),
            )
        }
    }
}

impl crate::Plugin for TeamSelectorCPlugin {
    fn self_check(&self) -> bool {
        unsafe {
            self.dll.get::<unsafe extern "C" fn(*const c_char, u64, *const i32, *const c_char) -> u8>(SELECT_TEAM_SYMBOL_NAME)
        }.is_ok()
    }
}
