use crate::{
    queries::{
        parameters_to_qeury, to_qeury_string, to_qeury_string_single, QeuryValue, ToQueryValue,
        ToQuerys,
    },
    shared_settings::{AnimtionType, Humor, MimeType, Nsfw},
};

pub enum GridStyle {
    Alternate,
    Blurred,
    WhiteLogo,
    Material,
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

pub enum GridDimentions {
    D460x215,
    D920x430,
    D600x900,
    D342x482,
    D660x930,
    D512x512,
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
}
