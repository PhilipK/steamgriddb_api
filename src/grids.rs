use crate::{
    author::Author,
    platforms::Platform,
    queries::{
        parameters_to_qeury, to_qeury_string, to_qeury_string_single, QeuryValue, ToQueryValue,
        ToQuerys,
    },
    result::SteamGridDbResult,
    shared_settings::{AnimtionType, Humor, MimeType, Nsfw},
};
use serde::{Deserialize, Serialize};
use serde_json::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GridStyle {
    #[serde(rename = "alternate")]
    Alternate,
    #[serde(rename = "blurred")]
    Blurred,
    #[serde(rename = "white_logo")]
    WhiteLogo,
    #[serde(rename = "material")]
    Material,
    #[serde(rename = "no_logo")]
    NoLogo,
}

impl ToQueryValue for GridStyle {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "styles".to_string(),
            value: match self {
                GridStyle::Alternate => "alternate",
                GridStyle::Blurred => "blurred",
                GridStyle::WhiteLogo => "white_logo",
                GridStyle::Material => "material",
                GridStyle::NoLogo => "no_logo",
            }
            .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub enum GridDimentions {
    #[serde(rename = "460x215")]
    D460x215,
    #[serde(rename = "920x430")]
    D920x430,
    #[serde(rename = "600x900")]
    D600x900,
    #[serde(rename = "342x482")]
    D342x482,
    #[serde(rename = "660x930")]
    D660x930,
    #[serde(rename = "512x512")]
    D512x512,
    #[serde(rename = "1024x1024")]
    D1024x1024,
}

impl ToQueryValue for GridDimentions {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "dimensions".to_string(),
            value: match self {
                GridDimentions::D460x215 => "460x215",
                GridDimentions::D920x430 => "920x430",
                GridDimentions::D600x900 => "600x900",
                GridDimentions::D342x482 => "342x482",
                GridDimentions::D660x930 => "660x930",
                GridDimentions::D512x512 => "512x512",
                GridDimentions::D1024x1024 => "1024x1024",
            }
            .to_string(),
        }
    }
}

pub fn get_grids_by_game_id_url(
    base_url: &str,
    game_id: &str,
    config: Option<&GridConfig>,
) -> String {
    get_grids_by_game_ids_url(base_url, &[game_id], config)
}

#[derive(Default)]
pub struct GridConfig<'a> {
    styles: Option<&'a [GridStyle]>,
    dimentions: Option<&'a [GridDimentions]>,
    mimes: Option<&'a [MimeType]>,
    types: Option<&'a [AnimtionType]>,
    nsfw: Option<&'a Nsfw>,
    humor: Option<&'a Humor>,
}

impl ToQuerys for GridConfig<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.dimentions),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}

pub fn get_grids_by_game_ids_url(
    base_url: &str,
    game_ids: &[&str],
    grid_config: Option<&GridConfig>,
) -> String {
    let game_ids_str = game_ids.join(",");
    let url_without_query = format!("{}/grids/game/{}", base_url, game_ids_str);
    match grid_config {
        Some(grid_config) => format!("{}?{}", url_without_query, grid_config.to_querys()),
        None => url_without_query,
    }
}

pub fn get_grids_by_platform_id_url(
    base_url: &str,
    platform: &Platform,
    game_ids: &str,
    grid_config: Option<&GridConfig>,
) -> String {
    get_grids_by_platform_ids_url(base_url, platform, &[game_ids], grid_config)
}

pub fn get_grids_by_platform_ids_url(
    base_url: &str,
    platform: &Platform,
    game_ids: &[&str],
    grid_config: Option<&GridConfig>,
) -> String {
    let game_ids_str = game_ids.join(",");
    let url_without_query = format!(
        "{}/grids/{}/{}",
        base_url,
        <&Platform as Into<String>>::into(platform),
        game_ids_str
    );
    match grid_config {
        Some(grid_config) => format!("{}?{}", url_without_query, grid_config.to_querys()),
        None => url_without_query,
    }
}

fn inner_response_to_publid(
    inner: InnerGridsResponse,
) -> SteamGridDbResult<Vec<SteamGridDbResult<Vec<Grid>>>> {
    if !inner.success {
        SteamGridDbResult::Error {
            errors: inner.errors,
            status: None,
        }
    } else {
        match inner.data {
            Some(data) => {
                let inner = data.iter().map(|i| {
                    if !i.success {
                        SteamGridDbResult::Error {
                            errors: i.errors.clone(),
                            status: Some(i.status),
                        }
                    } else {
                        match i.data.clone() {
                            Some(data) => SteamGridDbResult::Success(data),
                            None => SteamGridDbResult::Error {
                                status: None,
                                errors: Some(vec!["Could not parse resulting json".to_string()]),
                            },
                        }
                    }
                });
                SteamGridDbResult::Success(inner.collect())
            }
            None => SteamGridDbResult::Error {
                status: None,
                errors: Some(vec!["Could not parse resulting json".to_string()]),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InnerGridsResponse {
    pub success: bool,
    pub data: Option<Vec<InnerGridResponse>>,
    pub errors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InnerGridResponse {
    pub success: bool,
    pub status: u32,
    pub data: Option<Vec<Grid>>,
    pub errors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grid {
    pub id: u32,
    pub score: u32,
    pub style: GridStyle,
    pub width: u32,
    pub height: u32,
    pub nsfw: bool,
    pub humor: bool,
    pub notes: Option<String>,
    pub mime: MimeType,
    pub language: String,
    pub url: String,
    pub thumb: String,
    pub lock: bool,
    pub epilepsy: bool,
    pub upvotes: u32,
    pub downvotes: u32,
    pub author: Author,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_grids_by_game_ids_url_test_single_no_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_grids_by_game_ids_url(base_url, &["13136"], None);
        assert_eq!("https://www.steamgriddb.com/api/v2/grids/game/13136", url);
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_no_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_grids_by_game_ids_url(base_url, &["13136", "14065"], None);
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065",
            url
        );
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_styles_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridConfig::default();
        config.styles = Some(&[GridStyle::Alternate, GridStyle::Blurred]);
        let url = get_grids_by_game_ids_url(base_url, &["13136", "14065"], Some(&config));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065?styles=alternate,blurred",
            url
        );
    }

    #[test]
    fn get_grids_by_game_id_url_test_multiple_styles_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridConfig::default();
        config.styles = Some(&[GridStyle::Alternate, GridStyle::Blurred]);
        let actual = get_grids_by_game_id_url(base_url, "13136", Some(&config));
        let expected = get_grids_by_game_ids_url(base_url, &["13136"], Some(&config));
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_styles_multiple_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridConfig::default();
        config.styles = Some(&[GridStyle::Alternate, GridStyle::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        let url = get_grids_by_game_ids_url(base_url, &["13136", "14065"], Some(&config));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065?styles=alternate,blurred&types=animated,static",
            url
        );
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_styles_full_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridConfig::default();
        config.styles = Some(&[GridStyle::Alternate, GridStyle::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        config.humor = Some(&Humor::Any);
        config.nsfw = Some(&Nsfw::False);
        config.mimes = Some(&[MimeType::Jpeg, MimeType::Png]);
        config.dimentions = Some(&[GridDimentions::D1024x1024, GridDimentions::D920x430]);
        let url = get_grids_by_game_ids_url(base_url, &["13136", "14065"], Some(&config));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065?styles=alternate,blurred&dimensions=1024x1024,920x430&mimes=image/jpeg,image/png&types=animated,static&nsfw=false&humor=any",
            url
        );
    }

    #[test]
    fn get_grids_by_platform_ids_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridConfig::default();
        config.styles = Some(&[GridStyle::Alternate, GridStyle::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        config.humor = Some(&Humor::Any);
        config.nsfw = Some(&Nsfw::False);
        config.mimes = Some(&[MimeType::Jpeg, MimeType::Png]);
        config.dimentions = Some(&[GridDimentions::D1024x1024, GridDimentions::D920x430]);
        let url = get_grids_by_platform_ids_url(
            base_url,
            &Platform::EpicGameStore,
            &["13136", "14065"],
            Some(&config),
        );
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/egs/13136,14065?styles=alternate,blurred&dimensions=1024x1024,920x430&mimes=image/jpeg,image/png&types=animated,static&nsfw=false&humor=any",
            url
        );
    }

    #[test]
    fn parse_grids_test() {
        let json = std::fs::read_to_string("testdata/grids/grids.json").unwrap();
        let game_response: InnerGridsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, true);
        assert_eq!(game_response.data.is_some(), true);
        let data = game_response.data.unwrap();
        assert_eq!(data.len(), 2);
        let first_op = data.iter().next();
        let first = first_op.unwrap();
        let game_data_op = first.data.as_ref();
        assert_eq!(game_data_op.is_some(), true);
        let grids = game_data_op.unwrap();
        assert_eq!(grids.len(), 1);
        let first_grid = grids.iter().next().unwrap();
        assert_eq!(first_grid.id, 80200);
        assert_eq!(first_grid.nsfw, false);
    }

    #[test]
    fn parse_grids_with_error_test() {
        let json = std::fs::read_to_string("testdata/grids/grids_error.json").unwrap();
        let game_response: InnerGridsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, true);
        assert_eq!(game_response.data.is_some(), true);
        let data = game_response.data.unwrap();
        assert_eq!(data.len(), 2);
        let mut it = data.iter();
        it.next();
        let second_op = it.next();
        let second = second_op.unwrap();
        assert_eq!(false, second.success);
        assert_eq!(second.status, 404);
        assert_eq!(second.errors, Some(vec!["Game not found".to_string()]));
    }

    #[test]
    fn parse_grids_error_test() {
        let json = std::fs::read_to_string("testdata/grids/error.json").unwrap();
        let game_response: InnerGridsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, false);
        assert_eq!(game_response.data.is_some(), false);
        assert_eq!(
            game_response.errors,
            Some(vec!["Asset does not exist".to_string()])
        );
    }

    #[test]
    fn inner_response_to_publid_test() {
        let json = std::fs::read_to_string("testdata/grids/grids_error.json").unwrap();
        let game_response: InnerGridsResponse = serde_json::from_str(&json).unwrap();

        let games = inner_response_to_publid(game_response);
        assert_eq!(games.is_error(), false);
        if let SteamGridDbResult::Success(games) = games {
            assert_eq!(games.len(), 2);
            let mut it = games.iter();
            let first = it.next().unwrap();
            assert_eq!(first.is_error(), false);
            if let SteamGridDbResult::Success(games) = first {
                assert_eq!(games.len(), 1);
                let first_grid = games.iter().next().unwrap();
                assert_eq!(first_grid.id, 80200);
            }

            let next = it.next().unwrap();
            assert_eq!(next.is_error(), true);

            if let SteamGridDbResult::Error { errors, status } = first {
                assert_eq!(&Some(404), status);
                assert_eq!(&Some(vec!["Game not found".to_string()]), errors);
            }
        }
    }
}
