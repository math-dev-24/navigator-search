use async_trait::async_trait;
use crate::models::{item::Item, navigator::Navigator, query::Query};
use crate::search::{brave::BraveService, ddg::DuckDuckGoService};


#[async_trait]
pub trait SearchEngine {
    async fn search(&self, query: Query) -> Result<Vec<Item>, Box<dyn std::error::Error>>;
}

pub fn make_engine(navigator: Navigator) -> Box<dyn SearchEngine> {
    match navigator {
        Navigator::DuckDuckGo => Box::new(DuckDuckGoService::new()),
        Navigator::Brave => Box::new(BraveService::new()),
        _ => panic!("Not implemented"),
    }
}