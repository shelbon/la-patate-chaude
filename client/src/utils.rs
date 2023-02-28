use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use serde_json::Error;

use shared::messages::Message;
pub fn receive(mut stream: &TcpStream, mut array: [u8; 4]) -> Result<Message, Error> {
    let _ = stream.read_exact(array.as_mut());

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    let mut vector = vec![0; size_message];

    let _ = stream.read_exact(&mut vector);

    let message_received = str::from_utf8(&vector).unwrap();

    let welcome_serialized = serde_json::to_string(&message_received).unwrap();
    let a = welcome_serialized.replace('\\', "");

    println!("{}", message_received);

    let first_last_off: &str = &a[1..a.len() - 1];
    let message: Result<Message, Error> = serde_json::from_str(first_last_off);

    message
}

pub fn send(mut stream: &TcpStream, message_to_send: Message) {
    let serialized_message =
        serde_json::to_string(&message_to_send).expect("failed to serialize message");
    let len = serialized_message.len() as u32;
    // on écrit le préfixe (taille du prochain message)
    if let Err(_) = stream.write_all(&len.to_be_bytes()) {
        println!("Failed to send size of message")
    }
    println!("{:?}", serialized_message);
    if let Err(_e) = stream.write_all(serialized_message.as_bytes()) {
        println!("Failed to send message: {:?} ", serialized_message)
    }
}