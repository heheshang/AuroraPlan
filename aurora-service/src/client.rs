// use proto::{
//     ds_access_token::ds_access_token_service_client::DsAccessTokenServiceClient,
//     ds_user::{ds_user_service_client::DsUserServiceClient, GetDsUserRequest},
// };
// use tonic::{transport::Endpoint, Request};
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = Endpoint::from_static("http://0.0.0.0:50051");

//     /*
//     Client code is not implemented in completely
//     as it would just make the code base look too complicated ....
//     and interface requires a lot of boilerplate code to implement.

//     But a basic implementation is given below ....
//     please refer it to implement other ways to make your code pretty
//     */
//     let mut client: DsUserServiceClient<tonic::transport::Channel> =
//         DsUserServiceClient::connect(addr.clone()).await?;
//     let request = Request::new(GetDsUserRequest {
//         name: "admin".to_string(),
//     });
//     let response = client.get_ds_user(request).await?;

//     let mut client_acc = DsAccessTokenServiceClient::connect(addr.clone()).await?;

//     let resp = client_acc
//         .list_ds_access_tokens(Request::new(
//             proto::ds_access_token::ListDsAccessTokensRequest {
//                 page_size: 5,
//                 page_num: 1,
//             },
//         ))
//         .await?;

//     eprintln!("RESPONSE={:?}", response);
//     eprintln!("RESPONSE={:?}", resp);

//     Ok(())
// }
