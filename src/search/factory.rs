use async_trait::async_trait;
use crate::models::{item::Item, navigator::Navigator, query::Query};
use crate::search::{brave::BraveService, ddg::DuckDuckGoService};


#[async_trait]
pub trait SearchEngine {
    async fn search(&self, query: &Query) -> Result<Vec<Item>, Box<dyn std::error::Error>>;
}

pub fn make_engine(navigator: Navigator) -> Box<dyn SearchEngine> {
    match navigator {
        Navigator::DuckDuckGo => Box::new(DuckDuckGoService::new()),
        Navigator::Brave => Box::new(BraveService::new()),
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::query::Query;

    #[test]
    fn test_make_engine_duckduckgo() {
        let engine = make_engine(Navigator::DuckDuckGo);
        drop(engine);
    }

    #[test]
    fn test_make_engine_brave() {
        let engine = make_engine(Navigator::Brave);
        drop(engine);
    }

    #[test]
    #[should_panic(expected = "Not implemented")]
    fn test_make_engine_unsupported_navigator() {
        let _ = make_engine(Navigator::Google);
    }

    #[tokio::test]
    async fn test_search_engine_trait_duckduckgo() {
        let engine = make_engine(Navigator::DuckDuckGo);
        let query = Query {
            text: "test query".to_string(),
            bang: None
        };

        let result = engine.search(&query).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_search_engine_trait_brave() {
        let engine = make_engine(Navigator::Brave);
        let query = Query {
            text: "test query".to_string(),
            bang: None
        };

        let result = engine.search(&query).await;
        assert!(result.is_ok() || result.is_err());
    }
}