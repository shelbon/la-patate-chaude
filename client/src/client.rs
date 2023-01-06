use std::io::{Read, Write};
use std::net::TcpStream;
use shared::messages::Message;


pub fn init_talk_to_server(stream: &mut TcpStream) {

    let message = "\"Hello\"";
    println!("{}", message.len());
    let len = message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel
    handle_client(stream);
}

pub fn send_subscribe_message(stream:  &mut TcpStream,name:String) {
    let subscribe = Message::Subscribe {
        name
    };
    let message = serde_json::to_string(&subscribe).unwrap();
    println!("{:?}",message);
    let len = message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(&message.as_bytes()).unwrap();
    handle_client(stream);
}

pub fn handle_client( stream: &mut TcpStream) {
    let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
    stream.read_exact(buf_len.as_mut()).unwrap(); // lit exactement la taille du buffer

    let len = u32::from_be_bytes(buf_len); // convertit les 4 octets en un entier u32

    let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
    stream.read_exact(buf.as_mut()).unwrap(); // on remplit le buffer
    // c'est arrivé
    println!("{buf:?}"); // en octets
    let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
    println!("{s}"); // en String
}