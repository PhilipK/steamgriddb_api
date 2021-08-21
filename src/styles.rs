use serde::{Deserialize, Serialize};

use crate::queries::{QeuryValue, ToQueryValue};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Style{
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StyleLogo {
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "custom")]
    Custom,
    
}

impl ToQueryValue for StyleLogo {
    fn to_query_value(&self) -> QeuryValue {
        QeuryValue {
            name: "styles".to_string(),
            value: match self {                
                Official => "official",                 
                White => "white",                 
                Black => "black",                
                Custom => "custom",
                
            }
            .to_string(),
        }
    }
}
