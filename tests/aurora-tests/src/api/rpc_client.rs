use aurora_api::model;

use lib_common::core_results::results::Result;
use lib_proto::pb::ds_user::GetDsUserByIdRequest;
#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = model::client::service::_ds_user_service_client().await?;

    let req = tonic::Request::new(GetDsUserByIdRequest { id: 1 });
    let res = client.get_ds_user_by_id(req).await?;
    println!("res: {:?}", res);
    Ok(())
}
