use crate::{
    dimensions::{GridDimentions, HeroDimentions, IconDimensions},
    platforms::Platform,
    shared_settings::{AnimtionType, ImageType, MimeType, MimeTypeLogo, Nsfw},
};

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
            let name = items.first().unwrap().to_query_value().name;
            let value = items
                .iter()
                .map(ToQueryValue::to_query_value)
                .map(|x| x.value)
                .collect::<Vec<String>>()
                .join(",");
            Some(format!("{}={}", name, value))
        }
        _ => None,
    }
}

// Dimentions
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

// Platforms

impl Into<String> for &Platform {
    fn into(self) -> String {
        match self {
            Platform::Steam => "steam",
            Platform::Origin => "origin",
            Platform::EpicGameStore => "egs",
            Platform::BattleNet => "bnet",
            Platform::Uplay => "uplay",
            Platform::Flashpoint => "flashpoint",
        }
        .to_string()
    }
}

// Shared

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
