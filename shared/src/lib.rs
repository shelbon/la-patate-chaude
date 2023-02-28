use md5::Digest;

pub mod challenge;
pub mod messages;

#[derive(Debug)]
pub struct ServerConfig {
    pub server_address: String,
    pub name: String,
    pub port: Option<u16>,
}

impl ServerConfig {
    pub fn new(server_address: String, name: String, server_port: Option<u16>) -> Self {
        ServerConfig {
            server_address,
            name,
            port: server_port,
        }
    }
    pub fn full_server_address(&self) -> String {
        return match &self.port {
            Some(port) => format!("{}:{}", &self.server_address, port),
            None => {
                let server_address_split: Vec<&str> = self.server_address.split(':').collect();
                if server_address_split.len() > 1 {
                    self.server_address.clone()
                } else {
                    format!("{}:{}", self.server_address, "7878")
                }
            }
        };
    }
}
fn check_seed(binary_hash: String, complexity: u32) -> bool {
    for (index, character) in binary_hash.chars().enumerate() {
        if character == '1' && index < complexity as usize {
            return false;
        } else if index >= complexity as usize {
            return true;
        }
    }

    false
}
///convert seed to hex
fn convert_to_hex(seed: i32) -> String {
    format!("{:016X}", seed)
}
/// concat seed with message
fn concat_string(seed: String, message: &str) -> String {
    format!("{}{}\n", seed, message)
}
/// format digest to hex format
fn format_digest_to_hex(digest: Digest) -> String {
    format!("{:032X}", digest)
}
///  format  hashcode to binary
fn format_to_binary(hashcode: &String) -> String {
    hashcode.chars().map(to_binary).collect()
}
///  convert a string to binary representation
fn to_binary(character: char) -> String {
    let binary = match character {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };

    String::from(binary)
}