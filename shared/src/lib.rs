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

struct ServerConfig {
    server_address: String,
    name: &'static str,
}

impl ServerConfig {
    fn new(server_address: String, name: &'static str) -> Self {
        ServerConfig {
            server_address,
            name,
        }
    }


    // fn check_server_address(server_address: String) -> Result<&'static str, _> {
    //     if (server_address.len() <= 0) {
    //         Err(Error::new(ErrorKind::InvalidInput, "server_address can't be empty"))
    //     }
    //     if (server_address.len() > 255) {
    //         Err(Error::new(ErrorKind::InvalidInput, "server_address can't be more than 255 characters"))
    //     }
    //     if (!server_address.starts_with("http") || !server_address.starts_with("https")) {
    //         Err(Error::new(ErrorKind::InvalidInput, "server_address must start with http or https"))
    //     }
    //     Ok(("server address valid"))
    // }
}

struct Welcome{

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_failed_when_address_is_empty() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}