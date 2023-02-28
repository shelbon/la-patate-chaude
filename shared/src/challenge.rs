use crate::messages::{MD5HashCashInput, MD5HashCashOutput};
use crate::{check_seed, concat_string, convert_to_hex, format_digest_to_hex, format_to_binary};

pub trait Challenge {
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

pub struct HashCash {
    pub input: MD5HashCashInput,
}

impl Challenge for HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        HashCash { input }
    }

    fn solve(&self) -> Self::Output {
        let mut finish = false;
        let mut seed: u64 = 0;
        let mut hash_code = "".to_string();

        while !finish {
            let seed_in_hex = convert_to_hex(seed as i32);
            let seed_concat = concat_string(seed_in_hex.to_string(), &self.input.message);
            let digest = md5::compute(seed_concat);

            hash_code = format_digest_to_hex(digest);

            let binary_hash: String = format_to_binary(&hash_code);

            finish = check_seed(binary_hash, self.input.complexity);

            seed += 1;
        }

        MD5HashCashOutput {
            seed,
            hashcode: hash_code.parse().unwrap(),
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        check_seed(answer.hashcode.clone(), self.input.complexity)
    }
}

#[cfg(test)]
mod tests {
    use messages::{MD5HashCashOutput, Message};

    use crate::challenge::{Challenge, HashCash};
    use crate::messages;
    #[test]
    fn test_md5() {
        let input = messages::MD5HashCashInput {
            message: "Hello".to_string(),
            complexity: 9,
        };
        let hash_cash = HashCash::new(input);

        let md5output = hash_cash.solve();

        assert_eq!(hash_cash.verify(&md5output), true);
    }

    #[test]
    fn test_if_structure_welcome_is_good() {
        let welcome_message = Message::Welcome(messages::WelcomeInput { version: { 1 } });

        let check = equals_struct(welcome_message);

        let mes = "Welcome";

        assert_eq!(check, mes);
    }

    fn equals_struct(structure: Message) -> &'static str {
        let mut message = "";

        match structure {
            Message::Hello => message = "Hello",
            Message::Welcome(_) => message = "Welcome",
            Message::Subscribe(_) => message = "Subscribe",
            Message::SubscribeResult(_) => message = "SubscribeResult",
            Message::PublicLeaderBoard(_) => message = "PublicLeaderBoard",
            Message::Challenge(_) => message = "Challenge",
            Message::ChallengeResult(_) => message = "ChallengeResult",
            Message::RoundSummary(_) => message = "RoundSummary",
            Message::EndOfGame(_) => message = "EndOfGame",
            _ => {}
        }

        message
    }
}