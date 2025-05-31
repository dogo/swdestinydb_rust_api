mod card_response;
mod set_response;
mod handlers;

use axum::{
    routing::{get},
    Router,
};
use dotenv::dotenv;
use reqwest::Client;
use std::{net::SocketAddr, sync::Arc};
use std::env;
use tokio::net::TcpListener;

use handlers::{get_sets, get_card, get_set_cards, find_card};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_base_url = env::var("API_BASE_URL").expect("API_BASE_URL não definida");
    let client = Arc::new(Client::new());

    let state = AppState {
        client,
        api_base_url,
    };

    let app = Router::new()
        .route("/sets", get(get_sets))
        .route("/cards/:set_code", get(get_set_cards))
        .route("/card/:card_id", get(get_card))
        .route("/find", get(find_card))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Servidor rodando em http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
    api_base_url: String,
}