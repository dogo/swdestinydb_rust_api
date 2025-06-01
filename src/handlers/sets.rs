use axum::{
    extract::{State},
    response::IntoResponse,
    Json,
};

use crate::models::set_response::SetResponse;
use crate::services::sets_service::SetsService;
use crate::AppState;

#[utoipa::path(
    get,
    path = "/sets",
    responses(
        (status = 200, description = "Lista de sets", body = [SetResponse]),
        (status = 502, description = "Erro ao buscar sets")
    ),
    tag = "cards"
)]
pub(crate) async fn get_sets(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match SetsService::fetch_sets(&state).await {
        Ok(cards) => Json::<Vec<SetResponse>>(cards).into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_GATEWAY,
            e,
        ).into_response(),
    }
}