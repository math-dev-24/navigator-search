use std::error::Error;
use crate::models::item::Item;
use crate::models::query::Query;
use crate::search::factory::SearchEngine;

pub struct BraveService {
    base_url: String,
}

#[async_trait::async_trait]
impl SearchEngine for BraveService {
    async fn search(&self, query: Query) -> Result<Vec<Item>, Box<dyn Error>> {
        println!("{:?}", query);
        todo!()
    }
}

impl BraveService {
    pub fn new() -> Self {
        Self { base_url: String::from("https://search.brave.com/search") }
    }
}