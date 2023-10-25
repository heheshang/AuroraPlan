use aurora_common::core_results::results::GrpcRequest;
use proto::ds_alert_send_status::{
    ds_alert_send_status_service_server::DsAlertSendStatusService, CreateDsAlertSendStatusRequest,
    DeleteDsAlertSendStatusRequest, GetDsAlertSendStatusRequest, ListDsAlertSendStatusRequest,
    UpdateDsAlertSendStatusRequest,
};
// use sea_orm::entity::prelude::*;

use super::dao_service::AuroraRpcServer;

#[tonic::async_trait]
impl DsAlertSendStatusService for AuroraRpcServer {
    async fn list_ds_alert_send_status(
        &self,
        _req: GrpcRequest<ListDsAlertSendStatusRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::ListDsAlertSendStatusResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_send_status(
        &self,
        _req: GrpcRequest<GetDsAlertSendStatusRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::DsAlertSendStatus>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_alert_send_status(
        &self,
        _req: GrpcRequest<CreateDsAlertSendStatusRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::DsAlertSendStatus>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_alert_send_status(
        &self,
        _req: GrpcRequest<UpdateDsAlertSendStatusRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::DsAlertSendStatus>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_alert_send_status(
        &self,
        _req: GrpcRequest<DeleteDsAlertSendStatusRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
