use std::{io, str};
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde_json::Error;
use shared::messages::Message;
use shared::messages::Message::{Challenge, ChallengeResult, Subscribe};
use shared::messages::messages::{ChallengeAnswer, ChallengeOptions, ChallengeResultInput, MD5HashCashOutput, RecoverSecretInput, RecoverSecretOutput, SubscribeInput};
use crate::utils::utils::{receive,send};
fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let hello = Message::Hello;
            send(&mut stream, hello);

            let array = [0; 4];

            loop {
                let challenge = receive(&mut stream, array);

                match challenge {
                    Ok(v) => {
                        if let Message::Welcome(..) = v {
                            let subscribe = Message::Subscribe(SubscribeInput { name: "noura_patrick".parse().unwrap() });
                            send(&mut stream, subscribe);
                        }
                        if let Message::PublicLeaderBoard(..) = v {
                            println!("public")
                        }
                        if let Message::EndOfGame(..) = v {
                            break;
                        }
                        if let Message::Challenge(challenge) = v {
                            println!("challenge a effectué : {:?}", challenge);

                            loop {
                                match challenge.clone(){
                                    ChallengeOptions::RecoverSecret(recover) => {
                                        let recover_secret_answer =String::new(); //recover_secret(recover);

                                        let challenge_result = Message::ChallengeResult(ChallengeResultInput{answer:ChallengeAnswer::RecoverySecret(RecoverSecretOutput{secret_sentence:recover_secret_answer}),next_target:"patrick_noura".parse().unwrap()});
                                        send(&mut stream, challenge_result);

                                    },

                                    ChallengeOptions::MD5HashCash(hashcash) => {
                                        let complexity = hashcash.complexity;
                                        let message = hashcash.message;

                                        let md5answer =""; //md5hash_cash(complexity, message);

                                        //println!("reponse du challenge {:?}", md5answer);

                                        let challenge_result = Message::ChallengeResult(ChallengeResultInput { answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput{ seed:787877,hashcode:"".to_string() }), next_target: "patrick_noura".parse().unwrap() });
                                        send(&mut stream, challenge_result);

                                        break;
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {
                        println!("{:?}", challenge);
                        break;
                    }
                }
            }
        }
        Err(err) => panic!("Cannot connect: {}", err)
    }
}
