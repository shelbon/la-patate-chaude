use rand::Rng;
use shared::messages::messages::{
    Challenge, ChallengeAnswer, ChallengeResult, MD5HashCashOutput, PublicPlayer, Subscribe,
    SubscribeResult,
};
use shared::messages::Message;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message = serde_json::to_string(&message_to_send).expect("failed to serialize message");
    let len = message.len() as u32;
    stream
        .write_all(&len.to_be_bytes())
        .expect("Failed to send size"); // on écrit le préfixe (taille du prochain message)
    stream
        .write_all(message.as_bytes())
        .expect("Failed to send message"); // puis le message en tant que tel
}

pub fn handle_response_from_server(
    stream: &mut TcpStream,
    mut players: Vec<PublicPlayer>,
    connexion_name: &String,
) {
    let mut rng = rand::thread_rng();
    loop {
        let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
        stream
            .read_exact(buf_len.as_mut())
            .expect("Failed to get first message(size of the message)"); // lit exactement la taille du buffer

        let len = u32::from_be_bytes(buf_len); // convertit les 4 octets en un entier u32

        let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
        stream
            .read_exact(buf.as_mut())
            .expect("Failed to get the message"); // on remplit le buffer
                                                  // c'est arrivé
                                                  //println!("{buf:?}"); // en octets
        let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
                                               //println!("{s:?}");
        let message_deserialized =
            serde_json::from_str::<Message>(&s).expect("failed to deserialize message");
        //println!("{message_deserialized:?}");
        match message_deserialized {
            Message::Welcome(_) => {
                let subscribe = Message::Subscribe(Subscribe {
                    name: connexion_name.to_string(),
                });
                send(stream, subscribe);
            }
            Message::SubscribeResult(subscribe) => match subscribe {
                SubscribeResult::Ok => {}
                SubscribeResult::Err(error) => {
                    panic!("{:?}", error);
                }
            },
            Message::PublicLeaderBoard(public_leader_board) => {
                players.append(
                    &mut public_leader_board
                        .0
                        .clone()
                        .into_iter()
                        .filter(|p| p.name != *connexion_name)
                        .collect::<Vec<PublicPlayer>>(),
                );
            }
            Message::Challenge(challenge) => match challenge {
                Challenge::MD5HashCash(md5_hash_cash_input) => {
                    //TODO
                    let challenge_answer = ChallengeAnswer::MD5HashCash(MD5HashCashOutput {
                        seed: 000000,
                        hashcode: "".to_string(),
                    });
                    let index_player = rng.gen_range(0..players.len());

                    match players.get(index_player) {
                        Some(player) => {
                            let challenge_result = Message::ChallengeResult(ChallengeResult {
                                answer: challenge_answer,
                                next_target: player.name.clone(),
                            });
                            send(stream, challenge_result);
                        }
                        None => {
                            println!("some error happened");
                        }
                    }
                    break;
                }
            },
            Message::ChallengeTimeout(challenge_timeout) => {
                println!("{}", challenge_timeout.message);
                break;
            }
            Message::RoundSummary(round_summary) => {
                println!("{:?}", round_summary.chain);
                break;
            }
            Message::EndOfGame(end_of_game) => {
                todo!()
            }
            _ => {
                println!("Invalid message {:?}", message_deserialized);
                break;
            }
        }
    }
}