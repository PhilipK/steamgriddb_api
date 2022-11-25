use serde::{Deserialize, Serialize};

/// Query type for each image type
pub enum QueryType<'a> {
    /// Query for grid images
    Grid(Option<GridQueryParameters<'a>>),
    /// Query for hero images
    Hero(Option<HeroQueryParameters<'a>>),
    /// Query for logo images
    Logo(Option<LogoQueryParameters<'a>>),
    /// Query for icon images
    Icon(Option<IconQueryParameters<'a>>),
}

impl ToQuerys for QueryType<'_> {
    fn to_querys(&self) -> String {
        match self {
            QueryType::Grid(None)
            | QueryType::Hero(None)
            | QueryType::Logo(None)
            | QueryType::Icon(None) => "".to_string(),
            QueryType::Grid(Some(grid_query_parameters)) => grid_query_parameters.to_querys(),
            QueryType::Hero(Some(hero_query_parameters)) => hero_query_parameters.to_querys(),
            QueryType::Logo(Some(logo_query_parameters)) => logo_query_parameters.to_querys(),
            QueryType::Icon(Some(icon_query_parameters)) => icon_query_parameters.to_querys(),
        }
    }
}

#[derive(Default)]
/// Qeury parameters for hero images
pub struct HeroQueryParameters<'a> {
    /// The hero styles
    pub styles: Option<&'a [Style]>,
    // The Hero dimentions
    pub dimentions: Option<&'a [HeroDimentions]>,
    // The mime types
    pub mimes: Option<&'a [MimeType]>,
    // The image animation type
    pub types: Option<&'a [AnimtionType]>,
    /// If Not Safe For Work images are allowed
    pub nsfw: Option<&'a Nsfw>,
    /// If humor images are allowed
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for HeroQueryParameters<'_> {
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

#[derive(Default)]
/// Qeury parameters for grid images
pub struct GridQueryParameters<'a> {
    /// The grid styles
    pub styles: Option<&'a [Style]>,
    // The grid dimentions
    pub dimentions: Option<&'a [GridDimentions]>,
    // The mime types
    pub mimes: Option<&'a [MimeType]>,
    // The image animation type
    pub types: Option<&'a [AnimtionType]>,
    /// If Not Safe For Work images are allowed
    pub nsfw: Option<&'a Nsfw>,
    /// If humor images are allowed
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for GridQueryParameters<'_> {
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

#[derive(Default)]
/// Qeury parameters for logo images
pub struct LogoQueryParameters<'a> {
    /// The logo styles
    pub styles: Option<&'a [Style]>,
    // The logo dimentions
    pub mimes: Option<&'a [MimeTypeLogo]>,
    // The image animation type
    pub types: Option<&'a [AnimtionType]>,
    /// If Not Safe For Work images are allowed
    pub nsfw: Option<&'a Nsfw>,
    /// If humor images are allowed
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for LogoQueryParameters<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}

#[derive(Default)]
/// Qeury parameters for icon images
pub struct IconQueryParameters<'a> {
    /// The icon styles
    pub styles: Option<&'a [Style]>,
    // The icon dimentions
    pub mimes: Option<&'a [MimeTypeIcon]>,
    // The image animation type
    pub types: Option<&'a [AnimtionType]>,
    /// If Not Safe For Work images are allowed
    pub nsfw: Option<&'a Nsfw>,
    /// If humor images are allowed
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for IconQueryParameters<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Grid dimentions
pub enum GridDimentions {
    #[serde(rename = "460x215")]
    /// 460x215
    D460x215,
    #[serde(rename = "920x430")]
    /// 920x430
    D920x430,
    #[serde(rename = "600x900")]
    /// 600x900
    D600x900,
    #[serde(rename = "342x482")]
    /// 342x482
    D342x482,
    #[serde(rename = "660x930")]
    /// 660x930
    D660x930,
    #[serde(rename = "512x512")]
    /// 512x512
    D512x512,
    #[serde(rename = "1024x1024")]
    /// 1024x1024
    D1024x1024,
}

#[derive(Serialize, Deserialize, Debug)]
/// Hero dimentions
pub enum HeroDimentions {
    #[serde(rename = "1920x620")]
    /// 1920x620
    D1920x620,
    #[serde(rename = "3840x1240")]
    /// 3840x1240
    D3840x1240,
    #[serde(rename = "1600x650")]
    /// 1600x650
    D1600x650,
}

#[derive(Serialize, Deserialize, Debug)]
/// Icon dimentions
pub enum IconDimensions {
    #[serde(rename = "16x16")]
    /// 16x16
    D16x16,
    #[serde(rename = "20x20")]
    /// 20x20
    D20x20,
    #[serde(rename = "24x24")]
    /// 24x24
    D24x24,
    #[serde(rename = "28x28")]
    /// 28x28
    D28x28,
    #[serde(rename = "32x32")]
    /// 32x32
    D32x32,
    #[serde(rename = "40x40")]
    /// 40x40
    D40x40,
    #[serde(rename = "48x48")]
    /// 48x48
    D48x48,
    #[serde(rename = "54x54")]
    /// 54x54
    D54x54,
    #[serde(rename = "57x57")]
    /// 57x57
    D57x57,
    #[serde(rename = "60x60")]
    /// 60x60
    D60x60,
    #[serde(rename = "64x64")]
    /// 64x64
    D64x64,
    #[serde(rename = "72x72")]
    /// 72x72
    D72x72,
    #[serde(rename = "76x76")]
    /// 76x76
    D76x76,
    #[serde(rename = "80x80")]
    /// 80x80
    D80x80,
    #[serde(rename = "96x96")]
    /// 96x96
    D96x96,
    #[serde(rename = "114x114")]
    /// 114x114
    D114x114,
    #[serde(rename = "120x120")]
    /// 120x120
    D120x120,
    #[serde(rename = "128x128")]
    /// 128x128
    D128x128,
    #[serde(rename = "144x144")]
    /// 144x144
    D144x144,
    #[serde(rename = "152x152")]
    /// 152x152
    D152x152,
    #[serde(rename = "160x160")]
    /// 160x160
    D160x160,
    #[serde(rename = "180x180")]
    /// 180x180
    D180x180,
    #[serde(rename = "192x192")]
    /// 192x192
    D192x192,
    #[serde(rename = "194x194")]
    /// 194x194
    D194x194,
    #[serde(rename = "256x256")]
    /// 256x256
    D256x256,
    #[serde(rename = "512x512")]
    /// 512x512
    D512x512,
    #[serde(rename = "768x768")]
    /// 768x768
    D768x768,
    #[serde(rename = "1024x1024")]
    /// 1024x1024
    D1024x1024,
}

pub(crate) trait ToQuerys {
    fn to_querys(&self) -> String;
}

pub(crate) struct QeuryValue {
    pub name: String,
    pub value: String,
}

pub(crate) trait ToQueryValue {
    fn to_query_value(&self) -> QeuryValue;
}

pub(crate) fn parameters_to_qeury(parameters: &[Option<String>]) -> String {
    let strings = parameters
        .iter()
        .filter_map(|f| f.as_ref().map(|s| s.to_owned()))
        .collect::<Vec<String>>();
    if !strings.is_empty() {
        strings.join("&")
    } else {
        "".to_string()
    }
}

pub(crate) fn to_qeury_string_single<T>(item: Option<&T>) -> Option<String>
where
    T: ToQueryValue,
{
    item.map(|item| {
        let query_value = item.to_query_value();
        format!("{}={}", query_value.name, query_value.value)
    })
}

pub(crate) fn to_qeury_string<T>(items: Option<&[T]>) -> Option<String>
where
    T: ToQueryValue,
{
    match items {
        Some(items) if !items.is_empty() => {
            let first_op = items.first();
            match first_op {
                Some(first) => {
                    let name = first.to_query_value().name;
                    let value = items
                        .iter()
                        .map(ToQueryValue::to_query_value)
                        .map(|x| x.value)
                        .collect::<Vec<String>>()
                        .join(",");
                    Some(format!("{}={}", name, value))
                }
                None => None,
            }
        }
        _ => None,
    }
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

impl ToQueryValue for HeroDimentions {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "dimensions".to_string(),
            value: match self {
                HeroDimentions::D1920x620 => "1920x620",
                HeroDimentions::D3840x1240 => "3840x1240",
                HeroDimentions::D1600x650 => "1600x650",
            }
            .to_string(),
        }
    }
}

impl ToQueryValue for IconDimensions {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "dimensions".to_string(),
            value: match self {
                IconDimensions::D16x16 => "16x16",
                IconDimensions::D20x20 => "20x20",
                IconDimensions::D24x24 => "24x24",
                IconDimensions::D28x28 => "28x28",
                IconDimensions::D32x32 => "32x32",
                IconDimensions::D40x40 => "40x40",
                IconDimensions::D48x48 => "48x48",
                IconDimensions::D54x54 => "54x54",
                IconDimensions::D57x57 => "57x57",
                IconDimensions::D60x60 => "60x60",
                IconDimensions::D64x64 => "64x64",
                IconDimensions::D72x72 => "72x72",
                IconDimensions::D76x76 => "76x76",
                IconDimensions::D80x80 => "80x80",
                IconDimensions::D96x96 => "96x96",
                IconDimensions::D114x114 => "114x114",
                IconDimensions::D120x120 => "120x120",
                IconDimensions::D128x128 => "128x128",
                IconDimensions::D144x144 => "144x144",
                IconDimensions::D152x152 => "152x152",
                IconDimensions::D160x160 => "160x160",
                IconDimensions::D180x180 => "180x180",
                IconDimensions::D192x192 => "192x192",
                IconDimensions::D194x194 => "194x194",
                IconDimensions::D256x256 => "256x256",
                IconDimensions::D512x512 => "512x512",
                IconDimensions::D768x768 => "768x768",
                IconDimensions::D1024x1024 => "1024x1024",
            }
            .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Game platforms/stores
pub enum Platform {
    #[serde(rename = "steam")]
    /// steam
    Steam,
    #[serde(rename = "origin")]
    /// origin
    Origin,
    #[serde(rename = "egs")]
    /// egs
    EpicGameStore,
    #[serde(rename = "bnet")]
    /// bnet
    BattleNet,
    #[serde(rename = "uplay")]
    /// uplay
    Uplay,
    #[serde(rename = "flashpoint")]
    /// flashpoint
    Flashpoint,
    #[serde(rename = "gog")]
    /// bnet
    GoG,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Platform::Steam => "steam",
            Platform::Origin => "origin",
            Platform::EpicGameStore => "egs",
            Platform::BattleNet => "bnet",
            Platform::Uplay => "uplay",
            Platform::GoG => "gog",
            Platform::Flashpoint => "flashpoint",
        }
        .to_string()
    }
}

impl ToQueryValue for MimeType {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "mimes".to_string(),
            value: match self {
                MimeType::Png => "image/png",
                MimeType::Jpeg => "image/jpeg",
                MimeType::Webp => "image/webp",
            }
            .to_string(),
        }
    }
}

impl ToQueryValue for MimeTypeIcon {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "mimes".to_string(),
            value: match self {
                MimeTypeIcon::Png => "image/png",
                MimeTypeIcon::Icon => "image/vnd.microsoft.icon",
            }
            .to_string(),
        }
    }
}

impl ToQueryValue for MimeTypeLogo {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "mimes".to_string(),
            value: match self {
                MimeTypeLogo::Png => "image/png",
                MimeTypeLogo::Webp => "image/webp",
            }
            .to_string(),
        }
    }
}

impl ToQueryValue for ImageType {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "image_type".to_string(),
            value: match self {
                ImageType::Static => "static",
                ImageType::Animated => "animated",
            }
            .to_string(),
        }
    }
}

impl ToQueryValue for Nsfw {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "nsfw".to_string(),
            value: match self {
                Nsfw::True => "true",
                Nsfw::False => "false",
                Nsfw::Any => "any",
            }
            .to_string(),
        }
    }
}

impl ToQueryValue for AnimtionType {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "types".to_string(),
            value: match self {
                AnimtionType::Static => "static",
                AnimtionType::Animated => "animated",
            }
            .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Mime types
pub enum MimeTypeIcon {
    #[serde(rename = "image/png")]
    /// image/png
    Png,
    #[serde(rename = "image/vnd.microsoft.icon")]
    /// image/webp
    Icon,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Mime types
pub enum MimeType {
    #[serde(rename = "image/png")]
    /// image/png
    Png,
    #[serde(rename = "image/jpeg")]
    /// image/jpeg
    Jpeg,
    #[serde(rename = "image/webp")]
    /// image/webp
    Webp,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Mime types for logos
pub enum MimeTypeLogo {
    #[serde(rename = "image/png")]
    /// image/png
    Png,
    #[serde(rename = "image/webp")]
    /// image/webp
    Webp,
}

#[derive(Serialize, Deserialize, Debug)]
/// Image animation types
pub enum ImageType {
    Static,
    Animated,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Is the image Not Safe For Work
pub enum Nsfw {
    True,
    False,
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Is the image houmerous
pub enum Humor {
    True,
    False,
    Any,
}

impl ToQueryValue for Humor {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "humor".to_string(),
            value: match self {
                Humor::True => "true",
                Humor::False => "false",
                Humor::Any => "any",
            }
            .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Image animation types
pub enum AnimtionType {
    Static,
    Animated,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum StyleType {
    Normal(Style),
    Logo(StyleLogo),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Image style
pub enum Style {
    #[serde(rename = "alternate")]
    /// alternate
    Alternate,
    #[serde(rename = "blurred")]
    /// blurred
    Blurred,
    #[serde(rename = "white_logo")]
    /// white_logo
    WhiteLogo,
    #[serde(rename = "material")]
    /// material
    Material,
    #[serde(rename = "no_logo")]
    /// no_logo
    NoLogo,
}

impl ToQueryValue for Style {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "styles".to_string(),
            value: match self {
                Style::Alternate => "alternate",
                Style::Blurred => "blurred",
                Style::WhiteLogo => "white_logo",
                Style::Material => "material",
                Style::NoLogo => "no_logo",
            }
            .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StyleLogo {
    #[serde(rename = "official")]
    /// official
    Official,
    #[serde(rename = "white")]
    /// white
    White,
    #[serde(rename = "black")]
    /// black
    Black,
    #[serde(rename = "custom")]
    /// custom
    Custom,
}

impl ToQueryValue for StyleLogo {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "styles".to_string(),
            value: match self {
                StyleLogo::Official => "official",
                StyleLogo::White => "white",
                StyleLogo::Black => "black",
                StyleLogo::Custom => "custom",
            }
            .to_string(),
        }
    }
}
