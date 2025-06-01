use axum::{
    extract::{State},
    response::IntoResponse,
    Json,
};

use crate::models::set_response::SetResponse;
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