use reqwest::Client;
use std::{sync::Arc};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) client: Arc<Client>,
    pub(crate) api_base_url: String,
}