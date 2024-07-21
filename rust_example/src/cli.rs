use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use clap::Parser;
use crate::hello_world::HelloNumReq;

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    SayHello(SayHelloOptions),
    SayHelloNum(SayHelloNumOptions)
}

#[derive(Debug, Parser)]
struct SayHelloOptions {
    #[clap(long)]
    name: Option<String>,
}

#[derive(Debug, Parser)]
struct SayHelloNumOptions {
    #[clap(long)]
    num: i32,
}
pub mod hello_world {
    // 由于生成proto指定了outdir，所以用rust的include方式导入
    include!("./helloworld.rs");
//    tonic::include_proto!("helloworld");
}

async fn say_hello(opts: SayHelloOptions) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: opts.name.expect("name should not be null"),
    });
    let response = client.say_hello(request).await?;
    let msg = response.into_inner().message;
    println!("{}", msg);
//    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn say_hello_num(opts: SayHelloNumOptions) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloNumReq {
        num: opts.num,
    });

    let response = client.say_hello_num(request).await?;

    let msg = response.into_inner().num;
    assert!(msg == opts.num);
    println!("{}", msg);
   // println!("RESPONSE={:?}", response);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::parse();

    use Command::*;
    match opts.command {
        SayHello(opts) => say_hello(opts).await?,
        SayHelloNum(opts) => say_hello_num(opts).await?,
    };

    Ok(())
}
