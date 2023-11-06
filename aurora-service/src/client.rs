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
use std::{process, time::Duration};
use tonic::Request;
use tonic_health::pb::{health_client::HealthClient, HealthCheckRequest};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let conn = tonic::transport::Endpoint::new("http://0.0.0.0:50051")?
            .connect()
            .await?;

        // let request = tonic::Request::new(HelloRequest {
        //     name: "Tonic".into(),
        // });
        // let response = client.say_hello(request).await?;
        // println!("RESPONSE={:?}", response);
        let mut client = HealthClient::new(conn);
        let request = Request::new(HealthCheckRequest {
            service: "helloworld.Greeter".into(),
        });
        match client.check(request).await {
            Ok(response) => {
                let status = response.into_inner().status();
                match status {
                    tonic_health::pb::health_check_response::ServingStatus::Unknown => {
                        println!("Unknown!")
                    }
                    tonic_health::pb::health_check_response::ServingStatus::Serving => {
                        println!("Serving!")
                    }
                    tonic_health::pb::health_check_response::ServingStatus::NotServing => {
                        println!("Not Serving!")
                    }
                    tonic_health::pb::health_check_response::ServingStatus::ServiceUnknown => {
                        println!("ServiceUnknown!")
                    }
                }
            }
            Err(status) => {
                match status.code() {
                tonic::Code::Unimplemented =>
                    println!("error: this server does not implement the grpc health protocol (grpc.health.v1.Health): {}", status.message()),
                tonic::Code::DeadlineExceeded => println!("timeout: health rpc did not complete within 1 second"),
                _ => println!("error: health rpc failed: {}", status.message()),
            };
                const ERROR_CODE_RPC_FAILURE: i32 = 111;
                process::exit(ERROR_CODE_RPC_FAILURE);
            }
        }
    }
    // Ok(())
}
