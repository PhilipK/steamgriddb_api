use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum Platform{
    #[serde(rename = "steam")]
    Steam,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "egs")]
    EpicGameStore,
    #[serde(rename = "bnet")]
    BattleNet,
    #[serde(rename = "uplay")]
    Uplay,
    #[serde(rename = "flashpoint")]
    Flashpoint
}
