use super::dao_service::AuroraRpcServer;
use proto::ds_queue::ds_queue_service_server::DsQueueService;

#[tonic::async_trait]
impl DsQueueService for AuroraRpcServer {
    async fn list_ds_queues(
        &self,
        _req: tonic::Request<proto::ds_queue::ListDsQueuesRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::ListDsQueuesResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::GetDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        todo!()
    }

    async fn create_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::CreateDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        todo!()
    }

    async fn update_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::UpdateDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueue>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_queue(
        &self,
        _req: tonic::Request<proto::ds_queue::DeleteDsQueueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
