use std::io::Read;
use std::net::{TcpListener, TcpStream};

use shared::messages::messages::PublicPlayer;
use shared::messages::Message;

const PLAYERS: Vec<PublicPlayer> = vec![];
fn main() {
    println!("Starting server...");
    println!("Listening on port 7878");
    let listener = TcpListener::bind("127.0.0.1:7878");
    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        handle_message(&mut stream);
                    }
                    Err(e) => {
                        println!("Error accepting stream: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
fn handle_message(stream: &mut TcpStream) {
    let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
    stream
        .read_exact(buf_len.as_mut())
        .expect("Failed to get first message(size of the message)"); // lit exactement la taille du buffer

    let len = u32::from_be_bytes(buf_len); // convertit les 4 octets en un entier u32

    let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
    stream
        .read_exact(buf.as_mut())
        .expect("Failed to get the message");
    let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
                                           //println!("{s:?}");
    let message_deserialized =
        serde_json::from_str::<Message>(&s).expect("failed to deserialize message");
    match message_deserialized {
        Message::Hello => {
            println!("Someone said hello!");
        }
        _ => {
            println!("Someone said something else");
        }
    }
}
