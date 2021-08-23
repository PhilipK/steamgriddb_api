mod response;
mod search;
mod images;
mod shared_settings;
mod queries;
mod platforms;
mod games;
mod author;
mod styles;
mod dimensions;
mod query_parameters;


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


