pub use std::io::Read;
use std::net::TcpStream;

use rand::Rng;

use shared::messages::ChallengeOptions::MD5HashCash;
use shared::messages::Message::{Challenge, ChallengeResult, Subscribe};
use shared::messages::{
    ChallengeAnswer, ChallengeOptions, ChallengeResultInput, Message, PublicPlayer, SubscribeInput,
    SubscribeResult,
};

use crate::utils::{md5hash_cash, receive, send};

pub fn handle_response_from_server(
    stream: &TcpStream,
    mut players: Vec<PublicPlayer>,
    connexion_name: &String,
) {
    let mut rng = rand::thread_rng();
    loop {
        let buf_len = [0u8; 4]; // pour lire les 4 octets du u32
        let received_message = receive(stream, buf_len);
        let index_player = if players.is_empty() {
            0
        } else {
            rng.gen_range(0..players.len())
        };

        match received_message {
            Ok(successfulnesses_message) => match successfulnesses_message {
                Message::Welcome(_) => {
                    let subscribe = Subscribe(SubscribeInput {
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
                Challenge(challenge) => {
                    if let MD5HashCash(hash_cash) = challenge {
                        let complexity = hash_cash.complexity;
                        let message = hash_cash.message;

                        let md5answer = md5hash_cash(complexity, message);

                        match players.get(index_player) {
                            Some(player) => {
                                let challenge_result = ChallengeResult(ChallengeResultInput {
                                    answer: ChallengeAnswer::MD5HashCash(md5answer),
                                    next_target: player.name.clone(),
                                });
                                println!("challenge_result:{:?}", challenge_result);
                                send(stream, challenge_result);
                            }
                            None => {
                                println!("some error happened");
                            }
                        }
                    }
                }
                Message::ChallengeTimeout(challenge_timeout) => {
                    println!("{}", challenge_timeout.message);
                    break;
                }
                Message::RoundSummary(round_summary) => {
                    println!("{:?}", round_summary.chain);
                }
                Message::EndOfGame(end_of_game) => {
                    println!("{:?}", end_of_game);
                }
                _ => {
                    panic!("Invalid message {:?}", successfulnesses_message);
                }
            },
            Err(error) => {
                panic!("{:?}", error);
            }
        }
    }
}