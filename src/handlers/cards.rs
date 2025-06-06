use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use std::{collections::HashMap, time::Instant};

use crate::models::card_response::CardResponse;
use crate::services::card_service::CardService;
use crate::utils::metrics::record_metrics;
use crate::AppState;

#[utoipa::path(
    get,
    path = "/card/{card_id}",
    responses(
        (status = 200, description = "Detalhes da carta", body = CardResponse),
        (status = 502, description = "Erro ao buscar carta")
    ),
    params(
        ("card_id" = String, Path, description = "ID da carta")
    ),
    tag = "cards"
)]
pub(crate) async fn get_card(
    State(state): State<AppState>,
    Path(card_id): Path<String>,
) -> impl IntoResponse {
    let start = Instant::now();
    let endpoint = "get_card";

    match CardService::fetch_card(&state, &card_id).await {
        Ok(card) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "200", elapsed);
            Json::<CardResponse>(card).into_response()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "502", elapsed);
            (axum::http::StatusCode::BAD_GATEWAY, e).into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/cards/{set_code}",
    responses(
        (status = 200, description = "Lista de cartas do set", body = [CardResponse]),
        (status = 502, description = "Erro ao buscar cartas do set")
    ),
    params(
        ("set_code" = String, Path, description = "Código do set")
    ),
    tag = "cards"
)]
pub(crate) async fn get_set_cards(
    State(state): State<AppState>,
    Path(set_code): Path<String>,
) -> impl IntoResponse {
    let start = Instant::now();
    let endpoint = "get_set_cards";

    match CardService::fetch_set_cards(&state, &set_code).await {
        Ok(cards) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "200", elapsed);
            Json::<Vec<CardResponse>>(cards).into_response()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "502", elapsed);
            (axum::http::StatusCode::BAD_GATEWAY, e).into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/find",
    responses(
        (status = 200, description = "Lista de cartas encontradas", body = [CardResponse]),
        (status = 400, description = "Erro ao buscar cartas")
    ),
    params(
        ("q" = String, Query, description = "Consulta para busca de cartas")
    ),
    tag = "cards"
)]
pub(crate) async fn find_card(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let start = Instant::now();
    let endpoint = "find_card";

    let query = params.get("q").cloned().unwrap_or_default();

    match CardService::find_card(&state, &query).await {
        Ok(cards) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "200", elapsed);
            Json::<Vec<CardResponse>>(cards).into_response()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_secs_f64();
            record_metrics(endpoint, "400", elapsed);
            (axum::http::StatusCode::BAD_REQUEST, e).into_response()
        }
    }
}
