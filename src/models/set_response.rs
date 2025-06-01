use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
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
