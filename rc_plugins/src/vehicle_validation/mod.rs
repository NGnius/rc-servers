mod plugin;
pub use plugin::{ValidationResultCode, VehicleValidatorPlugin};

mod c_binding;
pub use c_binding::VehicleValidatorCPlugin;
