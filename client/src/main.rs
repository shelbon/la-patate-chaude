mod client;
use std::net::TcpStream;
use clap::Parser;
use crate::client::handle_client;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long)]
    server_address: String,
    #[clap(short, long)]
    connexion_name: String,
}

fn main() {
    let args = Args::parse();
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    client::init_talk_to_server(&mut stream);
    client::send_subscribe_message(&mut stream,args.connexion_name);
    let end_game =false;
    while !end_game {
        handle_client(&mut stream);
    }
}