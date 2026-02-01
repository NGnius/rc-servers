use super::account_json::UserData;

fn u64_negative_on_err<E>(result: Result<u64, E>) -> i64 {
    result.map(|x| x as i64).unwrap_or(-1)
}

#[async_trait::async_trait]
impl super::CommonUser for UserData {
    fn account_id(&self) -> i32 {
        self.account.id
    }

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

    async fn db_counters(&self) -> Vec<(&'static str, i64)> {
        let mut count_map = Vec::with_capacity(4);
        count_map.push(("users", u64_negative_on_err(self.db.user_count().await)));
        count_map.push(("garages", u64_negative_on_err(self.db.garage_count().await)));
        count_map.push(("games", u64_negative_on_err(self.db.game_count().await)));
        count_map
    }

    async fn currency(&self, ty: super::CurrencyType, op: super::CurrencyOp) -> Result<u64, polariton_server::operations::SimpleOpError> {
        self.currency_op(ty, op).await.map_err(|e| polariton_server::operations::SimpleOpError::with_message(
            crate::data::error_codes::WebServicesError::DatabaseError as i16,
            format!("Currency operation failed for user {}: {}", self.account.id, e),
        ))
    }


}
