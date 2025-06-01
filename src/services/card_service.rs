use crate::models::card_response::CardResponse;
use crate::AppState;

use tracing::{info, error};

pub(crate) struct CardService;

impl CardService {
    pub async fn fetch_card(state: &AppState, card_id: &str) -> Result<CardResponse, String> {
        let url = format!("{}/card/{}", state.api_base_url, card_id);
        let response = state.client.get(&url).send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();

        info!("🌐 Status da API: {}", status);
        info!("📦 JSON recebido:\n{}", text);

        serde_json::from_str::<CardResponse>(&text)
            .map_err(|e| {
                error!("❌ Erro de deserialização: {}", e);
                format!("Invalid JSON: {e}")
            })
    }

    pub async fn fetch_set_cards(state: &AppState, set_code: &str) -> Result<Vec<CardResponse>, String> {
        let url = format!("{}/cards/{}", state.api_base_url, set_code);
        let response = state.client.get(&url).send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();

        info!("🌐 Status da API: {}", status);
        info!("📦 JSON recebido:\n{}", text);

        serde_json::from_str::<Vec<CardResponse>>(&text)
            .map_err(|e| {
                error!("❌ Erro de deserialização: {}", e);
                format!("Invalid JSON: {e}")
            })
    }

    pub async fn find_card(state: &AppState, query: &str) -> Result<Vec<CardResponse>, String> {
        if query.trim().is_empty() {
            return Err("Parâmetro 'q' é obrigatório".to_string());
        }

        let url = format!("{}/find?q={}", state.api_base_url, query);
        let response = state.client.get(&url).send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();

        info!("🌐 Status da API: {}", status);
        info!("📦 JSON recebido:\n{}", text);

        serde_json::from_str::<Vec<CardResponse>>(&text)
            .map_err(|e| {
                error!("❌ Erro de deserialização: {}", e);
                format!("Invalid JSON: {e}")
            })
    }
}