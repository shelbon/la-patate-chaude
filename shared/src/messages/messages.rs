use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Subscribe { name: String },
    Welcome {
        version: u8
    },
}