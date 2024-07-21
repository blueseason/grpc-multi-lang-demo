use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest,HelloNumReq,HelloNumRes};

pub mod hello_world {
    //    tonic::include_proto!("helloworld");
    // 由于生成proto指定了outdir，所以用rust的include方式导入
    include!("./helloworld.rs");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn say_hello_num(
        &self,
        request: Request<HelloNumReq>,
    ) -> Result<Response<HelloNumRes>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloNumRes {
            num: request.into_inner().num,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
