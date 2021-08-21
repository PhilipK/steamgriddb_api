mod grids;
mod shared_settings;
mod queries;


pub struct SteamGridDBApi {
    config: InnerConfiguration,
}

impl SteamGridDBApi {
    fn new(auth_key: &str) -> Self {
        Self::with_config(Configuration {
            auth_key: auth_key.to_string(),
            ..Default::default()
        })
    }

    fn with_config(config: Configuration) -> Self {
        SteamGridDBApi {
            config: config.into(),
        }
    }
}

struct InnerConfiguration {
    auth_key: String,
    base_url: String,
}

impl From<Configuration> for InnerConfiguration {
    fn from(config: Configuration) -> Self {
        let default_base_url = "https://www.steamgriddb.com/api/v2";
        InnerConfiguration {
            auth_key: config.auth_key,
            base_url: config.base_url.unwrap_or(default_base_url.to_string()),
        }
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum ImageType {
    Boxart,
    Hero,
    Logo,
}

pub enum Dimentions {
    D600x900,
}

#[derive(Debug, Default)]
pub struct Configuration {
    pub auth_key: String,
    pub base_url: Option<String>,
}

pub fn get_search_url(base_url: &str, qeury: &str) -> String {
    use urlencoding::encode;
    format!("{}/search/autocomplete/{}", base_url, encode(qeury))
}

pub fn get_game_by_game_id_url(base_url: &str, game_id: &str) -> String {    
    format!("{}/games/id/{}", base_url, game_id)
}

pub fn get_game_by_steam_app_id_url(base_url: &str, steam_app_id: &str) -> String {    
    format!("{}/games/steam/{}", base_url, steam_app_id)
}



pub fn get_grids_by_game_platform_id_url(base_url: &str, game_id: &str) -> String {    
    format!("{}/grids/game/{}", base_url, game_id)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
