use super::account_json::UserData;

#[async_trait::async_trait]
impl super::CommonUser for UserData {
    async fn resolve_config_vehicle(&self, vehicle: &crate::persist::config::VehicleInfo, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, cpu_counter: &crate::cubes::CpuListParser) -> Result<super::ResolvedVehicle, polariton_server::operations::SimpleOpError> {
        self.resolve_vehicle(vehicle, factory, weapon_order, cpu_counter).await
    }
}
