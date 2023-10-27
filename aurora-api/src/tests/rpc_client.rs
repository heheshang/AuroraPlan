use crate::model;

use aurora_common::core_results::results::Result;
use aurora_proto::pb::ds_user::GetDsUserByIdRequest;

#[tokio::test]
async fn test_get_ds_user_by_id() -> Result<()> {
    let mut client = model::client::service::_ds_user_service_client().await?;

    let req = tonic::Request::new(GetDsUserByIdRequest { id: 1 });
    let res = client.get_ds_user_by_id(req).await?;
    println!("res: {:?}", res);
    Ok(())
}
