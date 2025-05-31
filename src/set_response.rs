use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetResponse {
    pub name: String,
    pub code: String,
    pub position: i64,
    pub available: String,
    pub known: i64,
    pub total: i64,
    pub url: String,
}
