mod client;
mod utils;
use clap::Parser;

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
    //let args = Args::parse();
    //let server_config = ServerConfig::new(args.server_address, args.connexion_name, args.port);
    //println!("{:?}",server_config);
    //let stream = TcpStream::connect(server_config.full_server_address());
    //let players: Vec<PublicPlayer> = vec![];
    println!("Hello, world!");


}