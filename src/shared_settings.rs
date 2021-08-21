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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MimeTypeLogo {
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/webp")]
    Webp,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageType {
    Static,
    Animated,
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

pub enum Nsfw {
    True,
    False,
    Any,
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

pub enum AnimtionType {
    Static,
    Animated,
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
