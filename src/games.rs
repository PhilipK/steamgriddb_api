use serde::{Deserialize, Serialize};

use crate::platforms::Platform;

pub fn get_game_by_game_id_url(base_url: &str, game_id: &str) -> String {
    format!("{}/games/id/{}", base_url, game_id)
}

pub fn get_game_by_steam_app_id_url(base_url: &str, steam_app_id: &str) -> String {
    format!("{}/games/steam/{}", base_url, steam_app_id)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub types: Vec<Platform>,
    pub release_date: Option<u32>,
    pub verified: bool,
}

pub type GameResponse = crate::response::Response<Game>;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_game_by_game_id_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_game_by_game_id_url(base_url, "13136");
        assert_eq!("https://www.steamgriddb.com/api/v2/games/id/13136", url);
    }

    #[test]
    fn get_game_by_steam_app_id_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_game_by_steam_app_id_url(base_url, "13136");
        assert_eq!("https://www.steamgriddb.com/api/v2/games/steam/13136", url);
    }
    #[test]
    fn parse_game_response_test() {
        let json = std::fs::read_to_string("testdata/games/game.json").unwrap();
        let game_response: GameResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(true, game_response.success);

        assert_eq!(true, game_response.data.is_some());
        assert_eq!(true, game_response.errors.is_none());
        let data = game_response.data.unwrap();
        assert_eq!(13136, data.id);
        assert_eq!("Celeste", data.name);
        assert_eq!(1516867200, data.release_date.unwrap());
        assert_eq!(true, data.verified);
    }

    #[test]
    fn parse_game_error_response_test() {
        let json = std::fs::read_to_string("testdata/games/error.json").unwrap();
        let game_response: GameResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(false, game_response.success);
        assert_eq!(true, game_response.errors.is_some());
        assert_eq!(vec!["Game not found"], game_response.errors.unwrap());
    }
}
