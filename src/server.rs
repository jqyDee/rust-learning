use rust_learning::{parser::parse_args, types::{Client, Server}};
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args_parsed = parse_args(env::args()).map_err(|e| eprintln!("Could not parse args: {}", e))?;

    let mut server_state = Server::from_args(&args_parsed);
    let listener = TcpListener::bind(server_state.sock_addr)
        .await
        .map_err(|e| eprintln!("Could not create TcpListener: {}", e))?;

    // Connection loop
    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .map_err(|e| eprintln!("Stream error {}", e))?;

        let client: Client = Client::from(
            server_state.next_id,
            String::from("Test user"),
            addr,
            server_state.tx.clone(),
            stream,
        );

        server_state.register_client(client).await;
    }
}
