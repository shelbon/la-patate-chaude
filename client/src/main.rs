use std::net::TcpStream;

use clap::Parser;

use shared::messages::{Message, PublicPlayer};
use shared::ServerConfig;

use crate::client::handle_response_from_server;
use crate::utils::send;

mod client;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    server_address: String,
    #[clap(short, long)]
    port: Option<u16>,
    #[clap(short, long)]
    connexion_name: String,
}

fn main() {
    let args = Args::parse();
    let server_config = ServerConfig::new(args.server_address, args.connexion_name, args.port);
    let stream = TcpStream::connect(server_config.full_server_address());
    let players: Vec<PublicPlayer> = vec![];
    match stream {
        Ok(stream) => {
            send(&stream, Message::Hello);
            handle_response_from_server(&stream, players, &server_config.name);
        }
        Err(e) => {
            panic!("Error while connecting to server: {}", e);
        }
    }
}