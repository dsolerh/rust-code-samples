use helloworld::{HelloRequest, greeter_client::GreeterClient};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let req = tonic::Request::new(HelloRequest {
        name: "Tonic".to_string(),
    });

    let resp = client.say_hello(req).await?;
    println!("RESPONSE:{:?}", resp);

    Ok(())
}
