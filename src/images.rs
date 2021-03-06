use crate::query_parameters::*;

use serde::{Deserialize, Serialize};

/// This image struct contains relevant image information for an image.
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
    /// The steamgriddb id of the image.
    pub id: u32,
    /// The name of the image.
    pub score: u32,
    /// The animation style of the image.
    pub style: StyleType,
    
    /// The width of the image
    pub width: u32,
    
    /// The height of the image.
    pub height: u32,

    /// Is this image Not Safe For Work?
    pub nsfw: bool,

    /// Is this image humorous? 
    pub humor: bool,

    /// A list of possible notes for the image
    pub notes: Option<String>,

    /// The mimetype of the image
    pub mime: MimeTypes,

    /// The language of the image
    pub language: String,

    /// The url of the image
    pub url: String,

    /// The url of the thumbnail
    pub thumb: String,

    /// Is this image locked?
    pub lock: bool,

    /// Is this game epilepsy triggering?
    pub epilepsy: bool,

    /// The amount of upvotes the image has
    pub upvotes: u32,

    /// The amount of downvotes the image has
    pub downvotes: u32,

    /// The author of the image
    pub author: Author,
}


#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum MimeTypes{
    Default(MimeType),
    Logo(MimeTypeLogo),
    Icon(MimeTypeIcon)
}

/// Get an URL to request images for one game given its stemagriddb id.
pub fn get_images_by_game_id_url(base_url: &str, game_id: usize, config: &QueryType) -> String {
    get_images_by_game_ids_url(base_url, &[game_id], config)
}

/// Get an URL to request an image for each of the given stemagriddb game ids.
pub fn get_images_by_game_ids_url(
    base_url: &str,
    game_ids: &[usize],
    config: &QueryType,
) -> String {
    let game_id_strings: Vec<String> = game_ids.iter().map(|id| format!("{}", id)).collect();
    let game_ids_str = game_id_strings.join(",");
    let query_type_str = match config {
        QueryType::Grid(_) => "grids",
        QueryType::Hero(_) => "heroes",
        QueryType::Logo(_) => "logos",
        QueryType::Icon(_) => "icons",
    };
    let url_without_query = format!("{}/{}/game/{}", base_url, query_type_str, game_ids_str);
    let query_string = config.to_querys();
    if query_string.is_empty() {
        url_without_query
    } else {
        format!("{}?{}", url_without_query, query_string)
    }
}


/// Get an URL to request an images for a game given a platform and a platform specific id.
pub fn get_images_by_platform_id_url(
    base_url: &str,
    platform: &Platform,
    game_id: &str,
    grid_config: &QueryType,
) -> String {
    get_images_by_platform_ids_url(base_url, platform, &[game_id], grid_config)
}

/// Get an URL to request an image for each of the given platform ids.
///
/// ### Examples
/// ```
/// use steamgriddb_api::images::*;
/// use steamgriddb_api::query_parameters::*;
/// let url = get_images_by_platform_ids_url("https://www.steamgriddb.com/api/v2", &Platform::Steam, &["107500", "107510"], &QueryType::Grid(None));
/// ```
pub fn get_images_by_platform_ids_url(
    base_url: &str,
    platform: &Platform,
    game_ids: &[&str],
    config: &QueryType,
) -> String {
    let game_ids_str = game_ids.join(",");
    let query_type_str = match config {
        QueryType::Grid(_) => "grids",
        QueryType::Hero(_) => "heroes",
        QueryType::Logo(_) => "logos",
        QueryType::Icon(_) => "icons",
    };
    let url_without_query = format!(
        "{}/{}/{}/{}",
        base_url,
        query_type_str,
        platform.to_string(),
        game_ids_str
    );
    let query_string = config.to_querys();
    if query_string.is_empty() {
        url_without_query
    } else {
        format!("{}?{}", url_without_query, query_string)
    }
}

pub(crate) type InnerImagesMultipleIdsResponse =
    crate::response::Response<Vec<crate::response::Response<Vec<Image>>>>;

pub(crate) type InnerImagesSingleIdResponse = crate::response::Response<Vec<Image>>;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Author of the image
///
/// Constains the basic information about the user that has uploaded an image.
pub struct Author {
    /// Name of the author
    pub name: String,
    /// Steam64 id of the user
    pub steam64: String,
    /// Optional url to the users avatar
    pub avatar: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::{
        query_parameters::{GridQueryParameters, HeroQueryParameters},
        response::{response_to_result, response_to_result_flat, SteamGridDbError},
    };

    use super::*;

    use QueryType::*;

    #[test]
    fn get_grids_by_game_ids_url_test_single_no_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_images_by_game_ids_url(base_url, &[13136], &Grid(None));
        assert_eq!("https://www.steamgriddb.com/api/v2/grids/game/13136", url);
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_no_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let url = get_images_by_game_ids_url(base_url, &[13136, 14065], &Grid(None));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065",
            url
        );
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_styles_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        let url = get_images_by_game_ids_url(base_url, &[13136, 14065], &Grid(Some(config)));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065?styles=alternate,blurred",
            url
        );
    }

    #[test]
    fn get_grids_by_game_id_url_test_multiple_styles_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        let grid_config = Grid(Some(config));
        let actual = get_images_by_game_id_url(base_url, 13136, &grid_config);
        let expected = get_images_by_game_ids_url(base_url, &[13136], &grid_config);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_styles_multiple_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        let url = get_images_by_game_ids_url(base_url, &[13136, 14065], &Grid(Some(config)));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065?styles=alternate,blurred&types=animated,static",
            url
        );
    }

    #[test]
    fn get_grids_by_game_ids_url_test_multiple_styles_full_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        config.humor = Some(&Humor::Any);
        config.nsfw = Some(&Nsfw::False);
        config.mimes = Some(&[MimeType::Jpeg, MimeType::Png]);
        config.dimentions = Some(&[GridDimentions::D1024x1024, GridDimentions::D920x430]);
        let url = get_images_by_game_ids_url(base_url, &[13136, 14065], &Grid(Some(config)));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/game/13136,14065?styles=alternate,blurred&dimensions=1024x1024,920x430&mimes=image/jpeg,image/png&types=animated,static&nsfw=false&humor=any",
            url
        );
    }

    #[test]
    fn get_heroes_by_game_ids_url_test_multiple_styles_full_config() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = HeroQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        config.humor = Some(&Humor::Any);
        config.nsfw = Some(&Nsfw::False);
        config.mimes = Some(&[MimeType::Jpeg, MimeType::Png]);
        config.dimentions = Some(&[HeroDimentions::D1600x650, HeroDimentions::D3840x1240]);
        let url = get_images_by_game_ids_url(base_url, &[13136, 14065], &Hero(Some(config)));
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/heroes/game/13136,14065?styles=alternate,blurred&dimensions=1600x650,3840x1240&mimes=image/jpeg,image/png&types=animated,static&nsfw=false&humor=any",
            url
        );
    }

    #[test]
    fn get_grids_by_platform_ids_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = GridQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        config.humor = Some(&Humor::Any);
        config.nsfw = Some(&Nsfw::False);
        config.mimes = Some(&[MimeType::Jpeg, MimeType::Png]);
        config.dimentions = Some(&[GridDimentions::D1024x1024, GridDimentions::D920x430]);
        let url = get_images_by_platform_ids_url(
            base_url,
            &Platform::EpicGameStore,
            &["Salt", "14065"],
            &Grid(Some(config)),
        );
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/grids/egs/Salt,14065?styles=alternate,blurred&dimensions=1024x1024,920x430&mimes=image/jpeg,image/png&types=animated,static&nsfw=false&humor=any",
            url
        );
    }

    #[test]

    fn get_heroes_by_platform_ids_url_test() {
        let base_url = "https://www.steamgriddb.com/api/v2";
        let mut config = HeroQueryParameters::default();
        config.styles = Some(&[Style::Alternate, Style::Blurred]);
        config.types = Some(&[AnimtionType::Animated, AnimtionType::Static]);
        config.humor = Some(&Humor::Any);
        config.nsfw = Some(&Nsfw::False);
        config.mimes = Some(&[MimeType::Jpeg, MimeType::Png]);
        config.dimentions = Some(&[HeroDimentions::D3840x1240]);
        let url = get_images_by_platform_ids_url(
            base_url,
            &Platform::BattleNet,
            &["13136", "14065"],
            &Hero(Some(config)),
        );
        assert_eq!(
            "https://www.steamgriddb.com/api/v2/heroes/bnet/13136,14065?styles=alternate,blurred&dimensions=3840x1240&mimes=image/jpeg,image/png&types=animated,static&nsfw=false&humor=any",
            url
        );
    }

    #[test]
    fn parse_grids_test() {
        let json = std::fs::read_to_string("testdata/grids/grids_fo_multiple_ids.json").unwrap();
        let game_response: InnerImagesMultipleIdsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, Some(true));
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
    fn parse_heroes_test() {
        let json = std::fs::read_to_string("testdata/heroes/heroes.json").unwrap();
        let game_response: InnerImagesSingleIdResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, Some(true));
        assert_eq!(game_response.data.is_some(), true);
        let data = game_response.data.unwrap();
        assert_eq!(data.len(), 18);
        let first_op = data.iter().next();
        let first = first_op.unwrap();
        assert_eq!(first.id, 25973);
        assert_eq!(first.nsfw, false);
    }

    #[test]
    fn parse_grids_with_error_test() {
        let json = std::fs::read_to_string("testdata/grids/grids_error.json").unwrap();
        let game_response: InnerImagesMultipleIdsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, Some(true));
        assert_eq!(game_response.data.is_some(), true);
        let data = game_response.data.unwrap();
        assert_eq!(data.len(), 2);
        let mut it = data.iter();
        it.next();
        let second_op = it.next();
        let second = second_op.unwrap();
        assert_eq!(Some(false), second.success);
        assert_eq!(second.status, Some(404));
        assert_eq!(second.errors, Some(vec!["Game not found".to_string()]));
    }

    #[test]
    fn parse_grids_error_test() {
        let json = std::fs::read_to_string("testdata/grids/error.json").unwrap();
        let game_response: InnerImagesMultipleIdsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, Some(false));
        assert_eq!(game_response.data.is_some(), false);
        assert_eq!(
            game_response.errors,
            Some(vec!["Asset does not exist".to_string()])
        );
    }

    #[test]
    fn parse_single_id_grid() {
        let json = std::fs::read_to_string("testdata/grids/grids_for_single_id.json").unwrap();
        let game_response: InnerImagesSingleIdResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, Some(true));
        assert_eq!(game_response.data.is_some(), true);
        if let Some(data) = game_response.data {
            assert_eq!(data.len(), 31);
        }
    }

    #[test]
    fn inner_response_single_to_public_test() {
        let json = std::fs::read_to_string("testdata/grids/grids_for_single_id.json").unwrap();
        let game_response: InnerImagesSingleIdResponse = serde_json::from_str(&json).unwrap();
        let grids = response_to_result(game_response);
        assert_eq!(grids.is_err(), false);
        if let Ok(grids) = grids {
            assert_eq!(grids.len(), 31);
        }
    }

    #[test]
    fn inner_response_multiple_to_publid_test() {
        let json = std::fs::read_to_string("testdata/grids/grids_error.json").unwrap();
        let game_response: InnerImagesMultipleIdsResponse = serde_json::from_str(&json).unwrap();
        let grids = response_to_result_flat(game_response);
        assert_eq!(grids.is_err(), false);
        if let Ok(grids) = grids {
            assert_eq!(grids.len(), 2);
            let mut it = grids.iter();
            let first = it.next().unwrap();
            assert_eq!(first.is_err(), false);
            if let Ok(first_grid) = first {
                assert_eq!(first_grid.id, 80200);
            }

            let next = it.next().unwrap();
            assert_eq!(next.is_err(), true);

            if let Err(SteamGridDbError { errors, status }) = first {
                assert_eq!(&Some(404), status);
                assert_eq!(&Some(vec!["Game not found".to_string()]), errors);
            }
        }
    }

    #[test]
    fn parse_single_id_icon() {
        let json = std::fs::read_to_string("testdata/icons/icons_for_single_id.json").unwrap();
        let game_response: InnerImagesSingleIdResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(game_response.success, Some(true));
        assert_eq!(game_response.data.is_some(), true);
        if let Some(data) = game_response.data {
            assert_eq!(data.len(), 2);
        }
    }
}
