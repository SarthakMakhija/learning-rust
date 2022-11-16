use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

use crate::greetings_server::mod_greetings::greetings_client::GreetingsClient;
use crate::greetings_server::mod_greetings::greetings_server::Greetings;
use crate::greetings_server::mod_greetings::greetings_server::GreetingsServer;
use crate::greetings_server::mod_greetings::GreetingsRequest;
use crate::greetings_server::mod_greetings::GreetingsResponse;

#[derive(Debug, Default)]
pub struct DefaultGreetingsServer {}

pub mod mod_greetings {
    tonic::include_proto!("greetings"); //package name
}

#[tonic::async_trait]
impl Greetings for DefaultGreetingsServer {
    async fn say_hello(&self, request: Request<GreetingsRequest>) -> Result<Response<GreetingsResponse>, Status> {
        let reply = mod_greetings::GreetingsResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_greetings() {
    let server_address = "[::1]:50051".parse().unwrap();
    let greetings_server = DefaultGreetingsServer::default();
    let (shutdown_signal_sender, mut shutdown_signal_receiver) = mpsc::channel(1);

    let shutdown_block = async move {
        shutdown_signal_receiver.recv().await.map(|_| ());
        return;
    };
    let server_handle = tokio::spawn(async move {
        Server::builder()
            .add_service(GreetingsServer::new(greetings_server))
            .serve_with_shutdown(server_address, shutdown_block)
            .await
            .expect("Failed in starting the server");
    });
    let client_handle = tokio::spawn(async move {
        let response = send_client_request().await.unwrap();
        let greetings_response: GreetingsResponse = response.into_inner();
        assert_eq!("Hello Learning Rust!", greetings_response.message);
        shutdown_signal_sender
            .send("shutdown")
            .await
            .expect("Failed in sending the shutdown signal");
    });

    server_handle.await.unwrap();
    client_handle.await.unwrap();
}

async fn send_client_request() -> Result<Response<GreetingsResponse>, Box<dyn std::error::Error>> {
    let mut client = GreetingsClient::connect("http://[::1]:50051/").await?;
    let request = Request::new(GreetingsRequest {
        name: "Learning Rust".into(),
    });
    let response = client.say_hello(request).await?;
    Ok(response)
}
