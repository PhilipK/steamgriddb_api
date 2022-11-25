use serde::de::DeserializeOwned;
use crate::response::Response;

use crate::{
    games::{get_game_by_steam_app_id_url, get_gameinfo_by_game_id_url, GameInfo},
    images::{
        get_images_by_game_id_url, get_images_by_game_ids_url, get_images_by_platform_id_url,
        get_images_by_platform_ids_url, Image, InnerImagesMultipleIdsResponse,
        InnerImagesSingleIdResponse,
    },
    query_parameters::{Platform, QueryType},
    response::{response_to_result, response_to_result_flat, SteamGridDbResult},
    search::{get_search_url, InnerSearchResult, SearchResult},
    steam_static::SteamStaticUrls,
};

/// This Client provides a convenient way to interact with the SteamGrid API.
///
/// The client calls the API using the [reqwest](https://crates.io/crates/reqwest) crate and parses the result using the [serde](https://crates.io/crates/serde) crate.
///
/// ### Examples
///
/// Searching for a game and getting images for it:
/// ```no_run
/// use steamgriddb_api::client::Client;
/// use steamgriddb_api::query_parameters::QueryType::Grid;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new("my_auth_key");
///     let games = client.search("Celeste").await?;
///     let first_game = games.iter().next().ok_or("No games found")?;
///     assert_eq!("Celeste", first_game.name);
///     let images = client.get_images_for_id(first_game.id, &Grid(None)).await?;
///     Ok(())
///  }
/// ```
pub struct Client {
    auth_key: String,
    base_url: String,
    #[cfg(feature = "async")]
    client: reqwest::Client,
    #[cfg(feature = "blocking")]
    client: reqwest::blocking::Client,
}

impl Client {
    /// Creates a new client with the given auth key.
    ///
    /// ### Examples
    /// ```
    /// use steamgriddb_api::client::Client;
    /// # fn main() {
    /// let client = Client::new("my_auth_key");
    /// assert_eq!("my_auth_key", client.get_auth_key());
    /// # }
    /// ```
    pub fn new<S>(auth_key: S) -> Self
    where
        S: Into<String>,
    {
        let default_base_url = "https://www.steamgriddb.com/api/v2";

        #[cfg(feature = "async")]
        let client = reqwest::Client::new();

        #[cfg(feature = "blocking")]
        let client = reqwest::blocking::Client::new();

        Self {
            auth_key: auth_key.into(),
            base_url: default_base_url.to_owned(),
            client,
        }
    }

    /// Sets the base url for the client.
    ///
    /// The default url is <https://www.steamgriddb.com/api/v2>
    ///
    /// ### Examples
    ///
    /// ```
    /// use steamgriddb_api::client::Client;
    /// # fn main() {
    /// let mut client = Client::new("my_auth_key");
    /// client.set_base_url("https://localhost:8080/api/v2");
    /// assert_eq!("https://localhost:8080/api/v2", client.base_url());
    /// # }
    /// ```
    pub fn set_base_url<S>(&mut self, base_url: S)
    where
        S: Into<String>,
    {
        self.base_url = base_url.into();
    }

    /// Gets the base url for the client.
    ///
    /// The default url is <https://www.steamgriddb.com/api/v2>
    ///
    /// ### Examples
    ///
    /// ```
    /// use steamgriddb_api::client::Client;
    /// # fn main() {
    /// let mut client = Client::new("my_auth_key");    
    /// assert_eq!("https://www.steamgriddb.com/api/v2", client.base_url());
    /// # }
    /// ```
    pub fn base_url(&'_ self) -> &'_ str {
        self.base_url.as_str()
    }

    /// Gets the auth key for the client.
    ///
    /// ### Examples
    ///
    /// ```
    /// use steamgriddb_api::client::Client;
    /// # fn main() {
    /// let client = Client::new("my_auth_key");
    /// assert_eq!("my_auth_key", client.get_auth_key());
    /// # }
    /// ```
    pub fn get_auth_key(&self) -> &'_ str {
        self.auth_key.as_str()
    }

    /// Sets the auth key for the client.
    ///
    /// ### Examples
    ///
    /// ```
    /// use steamgriddb_api::client::Client;
    /// # fn main() {
    /// let mut client = Client::new("my_auth_key");
    /// client.set_auth_key("another_key");
    /// assert_eq!("another_key", client.get_auth_key());
    /// # }
    /// ```
    pub fn set_auth_key<S>(&mut self, auth_key: S)
    where
        S: Into<String>,
    {
        self.auth_key = auth_key.into();
    }

    /// Fetches images given a game id and a query type.
    ///    
    /// ### Examples
    /// The Query type decides which kind of images to fetch.
    ///
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    /// use steamgriddb_api::query_parameters::QueryType::*;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");
    /// let grid_images = client.get_images_for_id(7993, &Grid(None)).await?;
    /// let hero_images = client.get_images_for_id(7993, &Hero(None)).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Query parameters can be given to specify which images to fetch.
    ///
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    /// use steamgriddb_api::query_parameters::GridDimentions::*;
    /// use steamgriddb_api::query_parameters::QueryType::*;
    /// use steamgriddb_api::query_parameters::GridQueryParameters;    
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");
    /// let mut parameters = GridQueryParameters::default();
    /// parameters.dimentions = Some(&[D600x900,D512x512]);
    /// let filtered_grid_images = client.get_images_for_id(7993, &Grid(Some(parameters))).await?;    
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_images_for_id(
        &self,
        game_id: usize,
        query: &QueryType<'_>,
    ) -> Result<Vec<Image>, Box<dyn std::error::Error>> {
        let url = get_images_by_game_id_url(self.base_url.as_str(), game_id, query);
        let response = self
            .make_request::<InnerImagesSingleIdResponse>(url.as_str())
            .await?;
        Ok(response_to_result(response)?)
    }

    #[cfg(feature = "blocking")]
    pub fn get_images_for_id(
        &self,
        game_id: usize,
        query: &QueryType<'_>,
    ) -> Result<Vec<Image>, Box<dyn std::error::Error>> {
        let url = get_images_by_game_id_url(self.base_url.as_str(), game_id, query);
        let response = self.make_request::<InnerImagesSingleIdResponse>(url.as_str())?;
        Ok(response_to_result(response)?)
    }
    /// Fetches images given a list game id's and a query type.
    ///
    /// The resulting list will be a SteamGridDbResult<Image> for each id.
    ///            
    /// ### Examples
    /// One image will be fetched for each id.
    ///
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    /// use steamgriddb_api::query_parameters::QueryType::*;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");
    /// let ids = [7993,5153400];
    /// let grid_images = client.get_images_for_ids(&ids, &Grid(None)).await?;
    /// assert_eq!(ids.len(), grid_images.len());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_images_for_ids(
        &self,
        game_id: &[usize],
        query: &QueryType<'_>,
    ) -> Result<Vec<SteamGridDbResult<Image>>, Box<dyn std::error::Error>> {
        if game_id.is_empty() {
            return Ok(Vec::new());
        }

        if game_id.len() == 1 {
            let single_result = self.get_images_for_id(game_id[0], query).await?;
            if !single_result.is_empty() {
                return Ok(vec![SteamGridDbResult::Ok(single_result[0].clone())]);
            }
        }

        let url = get_images_by_game_ids_url(self.base_url.as_str(), game_id, query);

        let resposse = self
            .make_request::<InnerImagesMultipleIdsResponse>(url.as_str())
            .await?;
        Ok(response_to_result_flat(resposse)?)
    }

    #[cfg(feature = "blocking")]
    pub fn get_images_for_ids(
        &self,
        game_id: &[usize],
        query: &QueryType<'_>,
    ) -> Result<Vec<SteamGridDbResult<Image>>, Box<dyn std::error::Error>> {
        if game_id.is_empty() {
            return Ok(Vec::new());
        }

        if game_id.len() == 1 {
            let single_result = self.get_images_for_id(game_id[0], query)?;
            if !single_result.is_empty() {
                return Ok(vec![SteamGridDbResult::Ok(single_result[0].clone())]);
            }
        }

        let url = get_images_by_game_ids_url(self.base_url.as_str(), game_id, query);

        let resposse = self.make_request::<InnerImagesMultipleIdsResponse>(url.as_str())?;
        Ok(response_to_result_flat(resposse)?)
    }

    /// Search for games given a search query.
    ///     
    /// The search query will be url encoded, so that it will be safe to use.           
    ///     
    /// ### Examples
    ///
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    /// use steamgriddb_api::query_parameters::QueryType::*;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");    
    /// let search_results = client.search("Celeste").await?;
    /// let first_result = search_results.iter().next().ok_or("None found")?;
    /// assert_eq!(first_result.name, "Celeste");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn search(
        &self,
        query: &str,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let url = get_search_url(self.base_url.as_str(), query);
        let response = self.make_request::<InnerSearchResult>(url.as_str()).await?;
        Ok(response_to_result(response)?)
    }

    #[cfg(feature = "blocking")]
    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let url = get_search_url(self.base_url.as_str(), query);
        let response = self.make_request::<InnerSearchResult>(url.as_str())?;
        Ok(response_to_result(response)?)
    }
    /// Fetches images given a platform type, a platform specific game id and a query type.
    ///    
    /// ### Examples
    ///    
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    /// use steamgriddb_api::query_parameters::Platform::*;
    /// use steamgriddb_api::query_parameters::QueryType::*;
    /// use steamgriddb_api::query_parameters::GridQueryParameters;    
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");    
    /// let platform = EpicGameStore;
    /// let epic_games_images = client.get_images_for_platform_id(&platform, "Salt", &Grid(None)).await?;    
    /// # Ok(())
    /// # }
    /// ```

    #[cfg(feature = "async")]
    pub async fn get_images_for_platform_id(
        &self,
        platform: &Platform,
        game_id: &str,
        qeury: &QueryType<'_>,
    ) -> Result<Vec<Image>, Box<dyn std::error::Error>> {
        let url = get_images_by_platform_id_url(self.base_url.as_str(), platform, game_id, qeury);
        let response = self
            .make_request::<InnerImagesSingleIdResponse>(url.as_str())
            .await?;
        Ok(response_to_result(response)?)
    }

    #[cfg(feature = "blocking")]
    pub fn get_images_for_platform_id(
        &self,
        platform: &Platform,
        game_id: &str,
        qeury: &QueryType<'_>,
    ) -> Result<Vec<Image>, Box<dyn std::error::Error>> {
        let url = get_images_by_platform_id_url(self.base_url.as_str(), platform, game_id, qeury);
        let response = self.make_request::<InnerImagesSingleIdResponse>(url.as_str())?;
        Ok(response_to_result(response)?)
    }
    /// Fetches images given a platform type, a platform specific game ids and a query type.
    ///    
    /// The resulting list will be a SteamGridDbResult<Image> for each id.
    ///    
    /// ### Examples
    ///    
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    /// use steamgriddb_api::query_parameters::Platform::*;
    /// use steamgriddb_api::query_parameters::QueryType::*;
    /// use steamgriddb_api::query_parameters::GridQueryParameters;    
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>>{
    /// let mut client = Client::new("my_auth_key");    
    /// let platform = EpicGameStore;
    /// let ids = ["Salt", "Turkey"];
    /// let epic_games_images = client.get_images_for_platform_ids(&platform, &ids, &Grid(None)).await?;  
    /// # Ok(())  
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_images_for_platform_ids(
        &self,
        platform: &Platform,
        game_id: &[&str],
        qeury: &QueryType<'_>,
    ) -> Result<Vec<SteamGridDbResult<Image>>, Box<dyn std::error::Error>> {
        let url = get_images_by_platform_ids_url(self.base_url.as_str(), platform, game_id, qeury);
        let resposse = self
            .make_request::<InnerImagesMultipleIdsResponse>(url.as_str())
            .await?;
        Ok(response_to_result_flat(resposse)?)
    }

    #[cfg(feature = "blocking")]
    pub fn get_images_for_platform_ids(
        &self,
        platform: &Platform,
        game_id: &[&str],
        qeury: &QueryType<'_>,
    ) -> Result<Vec<SteamGridDbResult<Image>>, Box<dyn std::error::Error>> {
        let url = get_images_by_platform_ids_url(self.base_url.as_str(), platform, game_id, qeury);
        let resposse = self.make_request::<InnerImagesMultipleIdsResponse>(url.as_str())?;
        Ok(response_to_result_flat(resposse)?)
    }
    /// Fetch information about a game given a game id.
    ///    
    /// ### Examples
    ///    
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");    
    /// let game_info = client.get_game_info_for_id(13136).await?;    
    /// assert_eq!(game_info.name, "Celeste");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_game_info_for_id(
        &self,
        game_id: usize,
    ) -> Result<GameInfo, Box<dyn std::error::Error>> {
        let url = get_gameinfo_by_game_id_url(self.base_url.as_str(), game_id);
        let response = self.make_request::<GameInfo>(url.as_str()).await?;
        Ok(response)
    }

    #[cfg(feature = "blocking")]
    pub fn get_game_info_for_id(
        &self,
        game_id: usize,
    ) -> Result<GameInfo, Box<dyn std::error::Error>> {
        let url = get_gameinfo_by_game_id_url(self.base_url.as_str(), game_id);
        let response = self.make_request::<GameInfo>(url.as_str())?;
        Ok(response)
    }
    /// Fetch information about a game given a steam game id.
    ///    
    /// ### Examples
    ///    
    /// ```no_run
    /// use steamgriddb_api::client::Client;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::new("my_auth_key");    
    /// let game_info = client.get_game_by_steam_app_id(361420).await?;    
    /// assert_eq!(game_info.name, "Astroneer");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_game_by_steam_app_id(
        &self,
        steam_app_id: usize,
    ) -> Result<GameInfo, Box<dyn std::error::Error>> {
        let url = get_game_by_steam_app_id_url(self.base_url.as_str(), steam_app_id);
        let response = self.make_request::<Response<GameInfo>>(url.as_str()).await?;
        let game_info = response.data.unwrap();
        Ok(game_info)
    }

    #[cfg(feature = "blocking")]
    pub fn get_game_by_steam_app_id(
        &self,
        steam_app_id: usize,
    ) -> Result<GameInfo, Box<dyn std::error::Error>> {
        let url = get_game_by_steam_app_id_url(self.base_url.as_str(), steam_app_id);
        let response = self.make_request::<GameInfo>(url.as_str())?;
        Ok(response)
    }

    #[cfg(feature = "async")]
    async fn make_request<'de, T>(&self, url: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .client
            .get(url)
            .bearer_auth(self.auth_key.as_str())
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    #[cfg(feature = "blocking")]
    fn make_request<'de, T>(&self, url: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .client
            .get(url)
            .bearer_auth(self.auth_key.as_str())
            .send()?
            .json::<T>()?)
    }

    /// Get a SteamStaticUrls that contains the expected urls for the official Steam store images.
    pub fn get_official_steam_images_static(steam_app_id: &str) -> SteamStaticUrls {
        SteamStaticUrls::new(steam_app_id)
    }

    /// Get a SteamStaticUrls that contains the expected urls for the official Steam store images.
    pub fn get_official_steam_images(&self, steam_app_id: &str) -> SteamStaticUrls {
        Self::get_official_steam_images_static(steam_app_id)
    }
}
