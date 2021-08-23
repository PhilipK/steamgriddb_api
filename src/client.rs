use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    images::{
        get_images_by_game_id_url, get_images_by_game_ids_url, Image,
        InnerImagesMultipleIdsResponse, InnerImagesSingleIdResponse,
    },
    query_parameters::QueryType,
    response::{response_to_result, response_to_result_flat, SteamGridDbResult},
    search::{get_search_url, InnerSearchResult, SearchResult},
};

pub struct Client {
    auth_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(auth_key: String) -> Self {
        let default_base_url = "https://www.steamgriddb.com/api/v2";
        let client = reqwest::Client::new();
        Self {
            auth_key: auth_key,
            base_url: default_base_url.to_owned(),
            client,
        }
    }

    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    pub fn get_base_url<'a>(&'a self) -> &'a str {
        self.base_url.as_str()
    }

    pub fn get_auth_key<'a>(&'a self) -> &'a str {
        self.auth_key.as_str()
    }

    pub fn set_auth_key(&mut self, auth_key: String) {
        self.auth_key = auth_key;
    }

    pub async fn get_images_for_id<'a>(
        &self,
        game_id: &str,
        config: &QueryType<'a>,
    ) -> Result<SteamGridDbResult<Vec<Image>>, Box<dyn std::error::Error>> {
        let url = get_images_by_game_id_url(self.base_url.as_str(), game_id, config);
        let resposse = self
            .make_request::<InnerImagesSingleIdResponse>(url.as_str())
            .await?;
        Ok(response_to_result(resposse))
    }

    pub async fn get_images_for_ids<'a>(
        &self,
        game_id: &[&str],
        config: &QueryType<'a>,
    ) -> Result<SteamGridDbResult<Vec<SteamGridDbResult<Image>>>, Box<dyn std::error::Error>> {
        let url = get_images_by_game_ids_url(self.base_url.as_str(), game_id, config);

        let resposse = self
            .make_request::<InnerImagesMultipleIdsResponse>(url.as_str())
            .await?;
        Ok(response_to_result_flat(resposse))
    }

    pub async fn search(
        &self,
        query: &str,
    ) -> Result<SteamGridDbResult<Vec<SearchResult>>, Box<dyn std::error::Error>> {
        let url = get_search_url(self.base_url.as_str(), query);
        let resposne = self.make_request::<InnerSearchResult>(url.as_str()).await?;
        Ok(response_to_result(resposne))
    }

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
}
