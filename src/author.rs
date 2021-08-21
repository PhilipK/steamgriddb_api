use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author{
    pub name: String,
    pub steam64: String,
    pub avatar: String,
}