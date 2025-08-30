use scraper::{Html, Selector};
use url::Url;
use crate::models::item::Item;
use crate::models::navigator::Navigator;
use crate::models::query::Query;
use crate::models::bang::BangMode;
use crate::utils::fetcher::fetch;
use crate::utils::parser::get_html;
use crate::search::factory::SearchEngine;

pub struct DuckDuckGoService {
    base_url: String,
}

#[async_trait::async_trait]
impl SearchEngine for DuckDuckGoService {
    async fn search(&self, query: &Query) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        if let Some(bang) = &query.bang {
            return match bang.mode {
                BangMode::Redirect => {
                    Ok(vec![Item {
                        title: format!("Redirect to {}", bang.name),
                        url: bang.url.clone(),
                        domain: Navigator::DuckDuckGo
                    }])
                },
                BangMode::Filter => {
                    let filtered_query = format!("{} {}", query.text, bang.url);
                    let url_with_query = self.build_url(&filtered_query);
                    let url = Url::parse(&url_with_query).unwrap();
                    let result = fetch(url).await.unwrap();
                    let html = get_html(result);
                    let items = DuckDuckGoService::get_items(html, "div.results_links");
                    Ok(items)
                }
            }
        }

        let url_with_query = self.build_url(&query.text);
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

    fn build_url(&self, search_text: &str) -> String {
        format!("{}{}", self.base_url, search_text)
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
                        url: format!("https:{}", url),
                        domain: Navigator::DuckDuckGo
                    });
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::bang::{Bang, BangMode};

    #[test]
    fn test_build_url_without_pagination() {
        let service = DuckDuckGoService::new();
        let query = Query {
            text: "rust programming".to_string(),
            bang: None
        };
        
        let url = service.build_url(&query.text);
        assert_eq!(url, "https://html.duckduckgo.com/html/?q=rust programming");
    }

    #[test]
    fn test_build_url_with_pagination_page_1() {
        let service = DuckDuckGoService::new();
        let query = Query {
            text: "rust programming".to_string(),
            bang: None
        };
        
        let url = service.build_url(&query.text);
        assert_eq!(url, "https://html.duckduckgo.com/html/?q=rust programming");
    }

    #[test]
    fn test_build_url_with_pagination_page_2() {
        let service = DuckDuckGoService::new();
        let query = Query {
            text: "rust programming".to_string(),
            bang: None
        };
        
        let url = service.build_url(&query.text);
        assert_eq!(url, "https://html.duckduckgo.com/html/?q=rust programming&s=10");
    }

    #[test]
    fn test_build_url_with_pagination_page_3_custom_limit() {
        let service = DuckDuckGoService::new();
        let query = Query {
            text: "rust programming".to_string(),
            bang: None
        };
        
        let url = service.build_url(&query.text);
        assert_eq!(url, "https://html.duckduckgo.com/html/?q=rust programming&s=40");
    }

    #[tokio::test]
    async fn test_search_with_redirect_bang_page_1() {
        let service = DuckDuckGoService::new();
        let query = Query {
            text: "rust programming".to_string(),
            bang: Some(Bang {
                name: "yt".to_string(),
                mode: BangMode::Redirect,
                url: "https://www.youtube.com/results?search_query=rust%20programming".to_string(),
            })
        };

        let result = service.search(&query).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "Redirect to yt");
        assert_eq!(result[0].url, "https://www.youtube.com/results?search_query=rust%20programming");
    }

    #[tokio::test]
    async fn test_search_with_redirect_bang_page_2() {
        let service = DuckDuckGoService::new();
        let query = Query {
            text: "rust programming".to_string(),
            bang: Some(Bang {
                name: "yt".to_string(),
                mode: BangMode::Redirect,
                url: "https://www.youtube.com/results?search_query=rust%20programming".to_string(),
            })
        };

        let result = service.search(&query).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_items_empty_html() {
        let html = Html::parse_document("<html><body></body></html>");
        let items = DuckDuckGoService::get_items(html, "div.results_links");
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_get_items_with_results() {
        let html_content = r#"
            <html>
                <body>
                    <div class="results_links">
                        <h2 class="result__title">
                            <a class="result__a" href="//example.com/page1">Test Title 1</a>
                        </h2>
                    </div>
                    <div class="results_links">
                        <h2 class="result__title">
                            <a class="result__a" href="//example.com/page2">Test Title 2</a>
                        </h2>
                    </div>
                </body>
            </html>
        "#;
        
        let html = Html::parse_document(html_content);
        let items = DuckDuckGoService::get_items(html, "div.results_links");
        
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Test Title 1");
        assert_eq!(items[0].url, "https://example.com/page1");
        assert_eq!(items[1].title, "Test Title 2");
        assert_eq!(items[1].url, "https://example.com/page2");
    }
}

