use std::error::Error;
use url::Url;
use crate::models::item::Item;
use crate::models::query::Query;
use crate::search::factory::SearchEngine;
use crate::utils::fetcher::fetch;
use crate::utils::parser::get_html;

pub struct BraveService {
    base_url: String,
}

#[async_trait::async_trait]
impl SearchEngine for BraveService {
    async fn search(&self, query: &Query) -> Result<Vec<Item>, Box<dyn Error>> {
        let url = Url::parse( format!("{}?q={}", self.base_url, query.text).as_str()).unwrap();
        let body = fetch(url).await.unwrap();
        let html = get_html(body);
        println!("{:?}", html);

        Ok(vec![])
    }
}

impl BraveService {
    pub fn new() -> Self {
        Self { base_url: String::from("https://search.brave.com/search") }
    }
}