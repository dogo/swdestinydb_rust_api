use axum::{
    extract::{State},
    response::IntoResponse,
    Json,
};

use std::{time::Instant};

use crate::models::set_response::SetResponse;
use crate::services::sets_service::SetsService;
use crate::utils::metrics::record_metrics;
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
    let start = Instant::now();
    let endpoint = "get_sets";

    match SetsService::fetch_sets(&state).await {
        Ok(sets) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "200", elapsed);
            Json::<Vec<SetResponse>>(sets).into_response()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "502", elapsed);
            (axum::http::StatusCode::BAD_GATEWAY, e).into_response()
        }
    }
}