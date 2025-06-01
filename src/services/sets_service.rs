use crate::models::set_response::SetResponse;
use crate::AppState;

pub(crate) struct SetsService;

impl SetsService {
    pub async fn fetch_sets(state: &AppState) -> Result<Vec<SetResponse>, String> {
        let url = format!("{}/sets", state.api_base_url);
        let response = state.client.get(&url).send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();

        println!("🌐 Status da API: {status}");
        println!("📦 JSON recebido:\n{text}");

        serde_json::from_str::<Vec<SetResponse>>(&text)
            .map_err(|e| {
                eprintln!("❌ Erro de deserialização: {e}");
                format!("Invalid JSON: {e}")
            })
    }
}