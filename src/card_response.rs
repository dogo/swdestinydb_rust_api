use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardResponse {
    pub sides: Option<Vec<String>>,
    #[serde(rename = "set_code")]
    pub set_code: String,
    #[serde(rename = "set_name")]
    pub set_name: String,
    #[serde(rename = "type_code")]
    pub type_code: String,
    #[serde(rename = "type_name")]
    pub type_name: String,
    #[serde(rename = "faction_code")]
    pub faction_code: String,
    #[serde(rename = "faction_name")]
    pub faction_name: String,
    #[serde(rename = "affiliation_code")]
    pub affiliation_code: String,
    #[serde(rename = "affiliation_name")]
    pub affiliation_name: String,
    #[serde(rename = "rarity_code")]
    pub rarity_code: String,
    #[serde(rename = "rarity_name")]
    pub rarity_name: String,
    pub subtypes: Option<Vec<Subtype>>,
    pub position: i64,
    pub code: String,
    pub ttscardid: Option<String>,
    pub name: String,
    pub subtitle: Option<String>,
    pub cost: Option<Value>,
    pub health:  Option<i64>,
    pub points:  Option<String>,
    pub text:  Option<String>,
    #[serde(rename = "deck_limit")]
    pub deck_limit: i64,
    pub flavor:  Option<String>,
    pub illustrator:  Option<String>,
    #[serde(rename = "is_unique")]
    pub is_unique: bool,
    #[serde(rename = "has_die")]
    pub has_die: bool,
    #[serde(rename = "has_errata")]
    pub has_errata: bool,
    #[serde(rename = "flip_card")]
    pub flip_card: bool,
    pub url: String,
    pub imagesrc: Option<String>,
    pub label: String,
    pub cp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtype {
    pub code: String,
    pub name: String,
}
