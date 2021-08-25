use crate::queries::{QeuryValue, ToQueryValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MimeType {
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/webp")]
    Webp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MimeTypeLogo {
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/webp")]
    Webp,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageType {
    Static,
    Animated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Nsfw {
    True,
    False,
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

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

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum AnimtionType {
    Static,
    Animated,
}
