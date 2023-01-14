mod client;

use crate::client::{handle_response_from_server, send};
use clap::Parser;
use shared::messages::messages::PublicPlayer;
use shared::messages::Message;
use shared::ServerConfig;
use std::net::TcpStream;

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
    println!("{:?}",server_config);
    let stream = TcpStream::connect(server_config.full_server_address());
    let players: Vec<PublicPlayer> = vec![];
    match stream {
        Ok(mut stream) => {
            send(&mut stream, Message::Hello);

            handle_response_from_server(&mut stream, players, &server_config.name);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}