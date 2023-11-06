use proto::helloworld::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{Request, Response, Status};
#[derive(Debug, Default)]
pub struct MyGreeter {}
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}
