use scraper::{Html, Selector};
use url::Url;
use crate::models::item::Item;
use crate::models::query::Query;
use crate::utils::fetcher::fetch;
use crate::utils::parser::get_html;
use crate::search::factory::SearchEngine;

pub struct DuckDuckGoService {
    base_url: String,
}

#[async_trait::async_trait]
impl SearchEngine for DuckDuckGoService {
    async fn search(&self, query: Query) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let url_with_query = format!("{}{}", self.base_url, query.text);
        let url = Url::parse(&url_with_query).unwrap();
        let result = fetch(url).await.unwrap();
        let html = get_html(result);

        let items = DuckDuckGoService::get_items(html, "div.results_links");

        Ok(items)
    }
}

impl DuckDuckGoService {
    pub fn new() -> Self {
        Self { base_url : "https://html.duckduckgo.com/html/?q=".to_string() }
    }

    fn get_items(html: Html, elem: &str) -> Vec<Item> {
        let container_selector = Selector::parse(elem).unwrap();
        let title_selector = Selector::parse("h2.result__title a.result__a").unwrap();

        let mut result: Vec<Item> = Vec::new();

        for container in html.select(&container_selector) {
            if let Some(link) = container.select(&title_selector).next() {
                let title = link.text().collect::<Vec<_>>().join(" ");
                if let Some(url) = link.value().attr("href") {
                    result.push(Item {
                        title,
                        url: url.to_string(),
                    });
                }
            }
        }
        result
    }
}





