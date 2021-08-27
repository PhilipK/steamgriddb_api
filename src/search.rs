use serde::{Deserialize, Serialize};

/// Get the url to search for the given query
pub fn get_search_url(base_url: &str, qeury: &str) -> String {
    use urlencoding::encode;
    format!("{}/search/autocomplete/{}", base_url, encode(qeury))
}

pub(crate) type InnerSearchResult = crate::response::Response<Vec<SearchResult>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// A search result from the search API
pub struct SearchResult {
    /// The name of the game
    pub name: String,
    /// The release date of the game
    pub release_date: Option<usize>,
    /// Is this game verified?
    pub verified:bool,    
    /// The id of the game
    pub id: usize,
    /// The platform types of this game
    pub types: Vec<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_search_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_search_url(base_url, "Assassin's Creed III");
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/search/autocomplete/Assassin%27s%20Creed%20III",
            url
        );
    }

    #[test]
    fn parse_search_results() {
        let json = std::fs::read_to_string("testdata/search/search.json").unwrap();
        let game_response: InnerSearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, true);
        assert_eq!(game_response.data.is_some(), true);
        assert_eq!(game_response.errors.is_none(), true);


        if let Some(data) = game_response.data {
            assert_eq!(data.len(), 15);

            let first_game = &data[0];
            assert_eq!(first_game.name, "Assassin's Creed");
            assert_eq!(first_game.release_date, Some(1207724400));
            assert_eq!(first_game.verified, true);
            assert_eq!(first_game.id, 1451);
            assert_eq!(first_game.types, vec!["steam", "gog", "uplay"]);
        }
    }
}
