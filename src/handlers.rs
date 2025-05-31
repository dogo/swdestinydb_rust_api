use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use std::{collections::HashMap};

use crate::card_response::CardResponse;
use crate::set_response::SetResponse;
use crate::AppState;

pub(crate) async fn get_sets(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let url = format!("{}/sets", state.api_base_url);

    match state.client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            println!("🌐 Status da API: {status}");
            println!("📦 JSON recebido:\n{text}");

            // Tenta desserializar
            match serde_json::from_str::<Vec<SetResponse>>(&text) {
                Ok(cards) => {
                    Json(cards).into_response()
                }
                Err(e) => {
                    eprintln!("❌ Erro de deserialização: {e}");
                    (
                        axum::http::StatusCode::BAD_GATEWAY,
                        format!("Invalid JSON: {e}"),
                    )
                        .into_response()
                }
            }
        }
        Err(_) => (
            axum::http::StatusCode::BAD_GATEWAY,
            "Failed to contact upstream",
        )
            .into_response(),
    }
}


pub(crate) async fn get_card(
    State(state): State<AppState>,
    Path(card_id): Path<String>
) -> impl IntoResponse {
    let url = format!("{}/card/{}", state.api_base_url, card_id);

    match state.client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            println!("🌐 Status da API: {status}");
            println!("📦 JSON recebido:\n{text}");

            // Tenta desserializar
            match serde_json::from_str::<CardResponse>(&text) {
                Ok(card) => {
                    Json(card).into_response()
                }
                Err(e) => {
                    eprintln!("❌ Erro de deserialização: {e}");
                    (
                        axum::http::StatusCode::BAD_GATEWAY,
                        format!("Invalid JSON: {e}"),
                    )
                        .into_response()
                }
            }
        }
        Err(_) => (
            axum::http::StatusCode::BAD_GATEWAY,
            "Failed to contact upstream",
        )
            .into_response(),
    }
}

pub(crate) async fn get_set_cards(
    State(state): State<AppState>,
    Path(set_code): Path<String>,
) -> impl IntoResponse {
    let url = format!("{}/cards/{}", state.api_base_url, set_code);

    match state.client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            println!("🌐 Status da API: {status}");
            println!("📦 JSON recebido:\n{text}");

            // Tenta desserializar
            match serde_json::from_str::<Vec<CardResponse>>(&text) {
                Ok(cards) => {
                    Json(cards).into_response()
                }
                Err(e) => {
                    eprintln!("❌ Erro de deserialização: {e}");
                    (
                        axum::http::StatusCode::BAD_GATEWAY,
                        format!("Invalid JSON: {e}"),
                    )
                        .into_response()
                }
            }
        }
        Err(_) => (
            axum::http::StatusCode::BAD_GATEWAY,
            "Failed to contact upstream",
        )
            .into_response(),
    }
}

pub(crate) async fn find_card(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let query = params.get("q").cloned().unwrap_or_default();

    if query.trim().is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Parâmetro 'q' é obrigatório".to_string(),
        )
            .into_response();
    }

    let url = format!("{}/find?q={}", state.api_base_url, query);

    println!("🔍 Consultando API com a query: {}", url);

    match state.client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            println!("🌐 Status da API: {status}");
            println!("📦 JSON recebido:\n{text}");

            match serde_json::from_str::<Vec<CardResponse>>(&text) {
                Ok(cards) => Json(cards).into_response(),
                Err(e) => {
                    eprintln!("❌ Erro de deserialização: {e}");
                    (
                        axum::http::StatusCode::BAD_GATEWAY,
                        format!("Erro no JSON: {e}"),
                    )
                        .into_response()
                }
            }
        }
        Err(_) => (
            axum::http::StatusCode::BAD_GATEWAY,
            "Erro ao consultar a API externa",
        )
            .into_response(),
    }
}