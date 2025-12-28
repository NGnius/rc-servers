use super::account_json::UserData;

#[async_trait::async_trait]
impl super::CommonUser for UserData {
    fn public_id(&self) -> &'_ str {
        &self.account.public_id
    }

    fn is_mod(&self) -> bool {
        self.perms.moderator
    }

    fn is_admin(&self) -> bool {
        self.perms.administrator
    }

    fn is_dev(&self) -> bool {
        self.perms.developer
    }

    fn is_royal(&self) -> bool {
        self.perms.royalty
    }

    fn is_banned(&self) -> bool {
        self.perms.banned
    }

    async fn resolve_config_vehicle(&self, vehicle: &crate::persist::config::VehicleInfo, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, cpu_counter: &crate::cubes::CpuListParser) -> Result<super::ResolvedVehicle, polariton_server::operations::SimpleOpError> {
        self.resolve_vehicle(vehicle, factory, weapon_order, cpu_counter).await
    }

    async fn db_metrics(&self) -> oj_rc_database::DatabaseMetrics {
        self.db.metrics().await
    }

    async fn currency(&self, ty: super::CurrencyType, op: super::CurrencyOp) -> Result<u64, polariton_server::operations::SimpleOpError> {
        self.currency_op(ty, op).await.map_err(|e| polariton_server::operations::SimpleOpError::with_message(
            crate::data::error_codes::WebServicesError::DatabaseError as i16,
            format!("Currency operation failed for user {}: {}", self.account.id, e),
        ))
    }
}
