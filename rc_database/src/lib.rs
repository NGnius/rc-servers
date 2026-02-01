mod migration;
pub use migration::Migrator;

pub mod schema;

mod wrapper;
pub use wrapper::Database;

mod metrics;
pub use metrics::DatabaseMetrics;

#[cfg(feature = "factory")]
mod factory_wrapper;
#[cfg(feature = "factory")]
pub use factory_wrapper::FactoryDatabase;

pub use sea_orm;
