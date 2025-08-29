use url::Url;

pub async fn fetch(url: Url) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUERY: &str = "rust";

    #[tokio::test]
    async fn test_fetch() {
        let url = Url::parse(format!("https://html.duckduckgo.com/html/?q={}", QUERY).as_str()).unwrap();
        let res = fetch(url).await;
        assert!(res.is_ok());
    }
}