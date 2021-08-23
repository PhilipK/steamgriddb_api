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
impl Into<String> for &Platform{
    fn into(self) -> String {
        match self {
            Platform::Steam => "steam",
            Platform::Origin => "origin",
            Platform::EpicGameStore => "egs",
            Platform::BattleNet => "bnet",
            Platform::Uplay => "uplay",
            Platform::Flashpoint => "flashpoint",
        }.to_string()
    }
}