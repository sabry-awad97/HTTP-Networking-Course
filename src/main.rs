use rand::thread_rng;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Client, Error};

#[derive(Debug, serde::Deserialize)]
struct Item {
    name: String,
}

struct ApiClient {
    api_key: String,
    client: Client,
}

impl ApiClient {
    fn new(api_key: String) -> ApiClient {
        let client = Client::new();
        ApiClient { api_key, client }
    }

    async fn get_item_data(&self) -> Result<Vec<Item>, Error> {
        let url = "https://api.boot.dev/v1/courses_rest_api/learn-http/items";
        let mut headers = HeaderMap::new();
        headers.insert("X-API-Key", HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let res = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<Vec<Item>>()
            .await?;
        Ok(res)
    }

    fn generate_key() -> String {
        let characters: &[u8] = b"ABCDEF0123456789";
        let mut rng = thread_rng();
        let key: String = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..characters.len());
                characters[idx] as char
            })
            .collect();
        key
    }

    async fn log_items(&self) -> Result<(), Error> {
        let items = self.get_item_data().await?;
        for item in &items {
            println!("{}", item.name);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = ApiClient::generate_key();
    let client = ApiClient::new(api_key);
    client.log_items().await?;
    Ok(())
}
