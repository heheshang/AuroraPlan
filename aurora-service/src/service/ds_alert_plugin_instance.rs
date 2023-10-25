use proto::ds_alert_plugin_instance::ds_alert_plugin_instance_service_server::DsAlertPluginInstanceService;
// use sea_orm::entity::prelude::*;

use super::dao_service::AuroraRpcServer;

#[tonic::async_trait]
impl DsAlertPluginInstanceService for AuroraRpcServer {
    async fn list_ds_alert_plugin_instances(
        &self,
        _request: tonic::Request<
            proto::ds_alert_plugin_instance::ListDsAlertPluginInstancesRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::ListDsAlertPluginInstancesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<proto::ds_alert_plugin_instance::GetDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::DsAlertPluginInstance>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<proto::ds_alert_plugin_instance::CreateDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::DsAlertPluginInstance>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<proto::ds_alert_plugin_instance::UpdateDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::DsAlertPluginInstance>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_alert_plugin_instance(
        &self,
        _req: tonic::Request<proto::ds_alert_plugin_instance::DeleteDsAlertPluginInstanceRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
