mod input_lockup;
pub use input_lockup::EditModeInputLockupWorkaround;

pub struct Workarounds {
    emil: std::sync::Arc<EditModeInputLockupWorkaround>,
}

impl Workarounds {
    pub fn new() -> Self {
        Self {
            emil: std::sync::Arc::new(EditModeInputLockupWorkaround::new())
        }
    }

    pub fn edit_mode_input_lockup(&self) -> std::sync::Arc<EditModeInputLockupWorkaround> {
        self.emil.clone()
    }
}
