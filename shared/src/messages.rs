use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeInput {
    pub version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Subscribe(SubscribeInput),
    Welcome(WelcomeInput),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(ChallengeOptions),
    ChallengeResult(ChallengeResultInput),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicLeaderBoard(pub Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeOptions {
    MD5HashCash(MD5HashCashInput),
    RecoverSecret(RecoverSecretInput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResultInput {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeTimeout {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    OK(Ok),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadResult {
    used_time: f64,
    next_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ok {
    used_time: f64,
    next_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverySecret(RecoverSecretOutput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard,
}
