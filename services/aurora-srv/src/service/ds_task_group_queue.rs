use super::dao_service::AuroraRpcServer;
use lib_proto::ds_task_group_queue::ds_task_group_queue_service_server::DsTaskGroupQueueService;

#[tonic::async_trait]
impl DsTaskGroupQueueService for AuroraRpcServer {
    async fn list_ds_task_group_queues(
        &self,
        _req: tonic::Request<lib_proto::ds_task_group_queue::ListDsTaskGroupQueuesRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::ds_task_group_queue::ListDsTaskGroupQueuesResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_group_queue(
        &self,
        _req: tonic::Request<lib_proto::ds_task_group_queue::GetDsTaskGroupQueueRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_task_group_queue::DsTaskGroupQueue>, tonic::Status> {
        todo!()
    }

    async fn create_ds_task_group_queue(
        &self,
        _req: tonic::Request<lib_proto::ds_task_group_queue::CreateDsTaskGroupQueueRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_task_group_queue::DsTaskGroupQueue>, tonic::Status> {
        todo!()
    }

    async fn update_ds_task_group_queue(
        &self,
        _req: tonic::Request<lib_proto::ds_task_group_queue::UpdateDsTaskGroupQueueRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::ds_task_group_queue::DsTaskGroupQueue>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_task_group_queue(
        &self,
        _req: tonic::Request<lib_proto::ds_task_group_queue::DeleteDsTaskGroupQueueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
