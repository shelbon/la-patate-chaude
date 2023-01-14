pub mod messages;

trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}
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
                let server_address_splitted: Vec<&str> = self.server_address.split(":").collect();
                if server_address_splitted.len() > 1 {
                    self.server_address.clone()
                } else {
                    format!("{}:{}", self.server_address, "7878")
                }
            }
        };
    }
}