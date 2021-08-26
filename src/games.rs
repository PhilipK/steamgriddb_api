use serde::{Deserialize, Serialize};

use crate::query_parameters::Platform;

/// Get the URL to get info about a game
pub fn get_gameinfo_by_game_id_url(base_url: &str, game_id: usize) -> String {
    format!("{}/games/id/{}", base_url, game_id)
}

/// Get the URL to get info about a game, given a steam app id
pub fn get_game_by_steam_app_id_url(base_url: &str, steam_app_id: usize) -> String {
    format!("{}/games/steam/{}", base_url, steam_app_id)
}

/// Information about a game
#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    /// The steamgriddb id for the game
    pub id: usize,
    /// The name of the game
    pub name: String,
    /// The platforms known for this game
    pub types: Vec<Platform>,
    /// The release date for this game
    pub release_date: Option<usize>,
    /// If this game has been verified
    pub verified: bool,
}



#[cfg(test)]
mod tests {

    use super::*;
    
    pub(crate) type GameResponse = crate::response::Response<GameInfo>;

    #[test]
    fn get_game_by_game_id_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_gameinfo_by_game_id_url(base_url, 13136);
        assert_eq!("https://www.steamgriddb.com/api/v2/games/id/13136", url);
    }

    #[test]
    fn get_game_by_steam_app_id_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_game_by_steam_app_id_url(base_url, 361420);
        assert_eq!("https://www.steamgriddb.com/api/v2/games/steam/361420", url);
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
