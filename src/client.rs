use crate::{
    images::{
        get_images_by_game_id_url, get_images_by_game_ids_url, Image,
        InnerImagesMultipleIdsResponse, InnerImagesSingleIdResponse,
    },
    query_parameters::QueryType,
    response::{response_to_result, response_to_result_flat, SteamGridDbResult},
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

    pub async fn fetch_images_for_id<'a>(
        &self,
        game_id: &str,
        config: &QueryType<'a>,
    ) -> Result<SteamGridDbResult<Vec<Image>>, Box<dyn std::error::Error>> {
        let url = get_images_by_game_id_url(self.base_url.as_str(), game_id, config);
        let inner_response = self
            .client
            .get(url)
            .bearer_auth(self.auth_key.as_str())
            .send()
            .await?
            .json::<InnerImagesSingleIdResponse>()
            .await?;
        Ok(response_to_result(inner_response))
    }

    pub async fn fetch_images_for_ids_url<'a>(
        &self,
        game_id: &[&str],
        config: &QueryType<'a>,
    ) -> Result<SteamGridDbResult<Vec<SteamGridDbResult<Image>>>, Box<dyn std::error::Error>> {
        let url = get_images_by_game_ids_url(self.base_url.as_str(), game_id, config);
        let inner_response = self
            .client
            .get(url)
            .bearer_auth(self.auth_key.as_str())
            .send()
            .await?
            .json::<InnerImagesMultipleIdsResponse>()
            .await?;
        Ok(response_to_result_flat(inner_response))
    }
}
