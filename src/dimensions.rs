use serde::{Deserialize, Serialize};

use crate::queries::{QeuryValue, ToQueryValue};

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

#[derive(Serialize, Deserialize, Debug)]
pub enum HeroDimentions {
    #[serde(rename = "1920x620")]
    D1920x620,
    #[serde(rename = "3840x1240")]
    D3840x1240,
    #[serde(rename = "1600x650")]
    D1600x650,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum IconDimensions {
    #[serde(rename = "16x16")]
    D16x16,
    #[serde(rename = "20x20")]
    D20x20,
    #[serde(rename = "24x24")]
    D24x24,
    #[serde(rename = "28x28")]
    D28x28,
    #[serde(rename = "32x32")]
    D32x32,
    #[serde(rename = "40x40")]
    D40x40,
    #[serde(rename = "48x48")]
    D48x48,
    #[serde(rename = "54x54")]
    D54x54,
    #[serde(rename = "57x57")]
    D57x57,
    #[serde(rename = "60x60")]
    D60x60,
    #[serde(rename = "64x64")]
    D64x64,
    #[serde(rename = "72x72")]
    D72x72,
    #[serde(rename = "76x76")]
    D76x76,
    #[serde(rename = "80x80")]
    D80x80,
    #[serde(rename = "96x96")]
    D96x96,
    #[serde(rename = "114x114")]
    D114x114,
    #[serde(rename = "120x120")]
    D120x120,
    #[serde(rename = "128x128")]
    D128x128,
    #[serde(rename = "144x144")]
    D144x144,
    #[serde(rename = "152x152")]
    D152x152,
    #[serde(rename = "160x160")]
    D160x160,
    #[serde(rename = "180x180")]
    D180x180,
    #[serde(rename = "192x192")]
    D192x192,
    #[serde(rename = "194x194")]
    D194x194,
    #[serde(rename = "256x256")]
    D256x256,
    #[serde(rename = "512x512")]
    D512x512,
    #[serde(rename = "768x768")]
    D768x768,
    #[serde(rename = "1024x1024")]
    D1024x1024,
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
