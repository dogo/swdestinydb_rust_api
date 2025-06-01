mod handlers;
mod models;
mod services;
mod app_state;

use axum::{
    routing::{get},
    Router,
};
use dotenv::dotenv;
use reqwest::Client;
use std::{net::SocketAddr, sync::Arc};
use std::env;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing_subscriber;

use handlers::{get_sets, get_card, get_set_cards, find_card};
use app_state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let api_base_url = env::var("API_BASE_URL").expect("API_BASE_URL não definida");
    let client = Arc::new(Client::new());

    let state = AppState {
        client,
        api_base_url,
    };

    let app = Router::new()
        .route("/v1/sets", get(get_sets))
        .route("/v1/cards/:set_code", get(get_set_cards))
        .route("/v1/card/:card_id", get(get_card))
        .route("/v1/find", get(find_card))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Servidor rodando em http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_sets,
        handlers::get_card,
        handlers::get_set_cards,
        handlers::find_card
    ),
    components(
        schemas(
            models::set_response::SetResponse,
            models::card_response::CardResponse,
            models::card_response::Subtype
        )
    ),
    tags(
        (name = "cards", description = "Operações relacionadas a cartas")
    )
)]
pub struct ApiDoc;