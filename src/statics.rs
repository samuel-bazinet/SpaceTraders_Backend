use std::sync::OnceLock;

use reqwest::Client;
static CLIENT_LOCK: OnceLock<Client> = OnceLock::new();

pub fn get_client() -> &'static Client {
    CLIENT_LOCK.get_or_init(|| Client::new())
}
