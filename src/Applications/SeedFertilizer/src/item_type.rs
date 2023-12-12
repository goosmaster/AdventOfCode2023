#![allow(clippy::inherent_to_string)]

#[derive(Debug, PartialEq)]
pub enum ItemType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl ItemType {
    pub fn to_string(&self) -> String {
        return match self {
            ItemType::Seed => "Seed",
            ItemType::Soil => "Soil",
            ItemType::Fertilizer => "Fertilizer",
            ItemType::Water => "Water",
            ItemType::Light => "Light",
            ItemType::Temperature => "Temperature",
            ItemType::Humidity => "Humidity",
            ItemType::Location => "Location",
        }.to_string();
    }

    pub fn from_string(mut string: &str) -> ItemType {
        if string.ends_with("map") {
            let start_bytes = string.find("-to-").unwrap_or(0) + "-to-".len();
            let end_bytes = string.find(" map").unwrap_or(string.len());
            string = &string[start_bytes..end_bytes];
        }

        return match string.to_lowercase().as_str() {
            "seed" | "seeds" => ItemType::Seed,
            "soil" => ItemType::Soil,
            "fertilizer" => ItemType::Fertilizer,
            "water" => ItemType::Water,
            "light" => ItemType::Light,
            "temperature" => ItemType::Temperature,
            "humidity" => ItemType::Humidity,
            "location" => ItemType::Location,
            _ => {
                panic!("Unknown item type {}", string)
            }
        };
    }
}