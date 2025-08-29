use crate::models::bang::{Bang, BangMode};
use urlencoding::encode;


pub fn get_bang_with_query(question: &String, bang: &String) -> Option<Bang> {

    let question = encode(question);

    match bang.as_str() {
        "ia" => Some(Bang {
            name: "ia".to_string(),
            mode: BangMode::Redirect,
            url: format!("https://www.perplexity.ai/search/?q={}", question.to_string())
        }),
        "gh" => Some(Bang {
            name: "gh".to_string(),
            mode: BangMode::Redirect,
            url: format!("https://github.com/search?q={}", question.to_string())
        }),
        "d" => Some(Bang {
            name: "d".to_string(),
            mode: BangMode::Redirect,
            url: format!("https://duckduckgo.com/?q={}", question.to_string())
        }),
        "yt" => Some(Bang {
            name: "yt".to_string(),
            mode: BangMode::Redirect,
            url: format!("https://www.youtube.com/results?search_query={}", question.to_string())
        }),
        "tr" => Some(Bang {
            name: "tr".to_string(),
            mode: BangMode::Redirect,
            url: format!("https://translate.google.com/?sl=auto&tl=en&text={}", question.to_string())
        }),
        "r" => Some(Bang {
            name: "r".to_string(),
            mode: BangMode::Filter,
            url: "site:reddit.com".to_string()
        }),
        "w" => Some(Bang {
            name: "w".to_string(),
            mode: BangMode::Filter,
            url: "site:stackoverflow.com OR site:github.com".to_string()
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_url_bang_with_query() {

        let question = String::from("how to use rust");
        let question_encoded = encode(&question);

        let bang = get_bang_with_query(&question, &String::from("ia"));

        assert_eq!(bang, Some(Bang {
            name: "ia".to_string(),
            mode: BangMode::Redirect,
            url: format!("https://www.perplexity.ai/search/?q={}", question_encoded.to_string())
        }));
    }

    #[test]
    fn test_get_url_bang_with_query_no_bang() {
        let question = String::from("how to use rust");
        let bang = get_bang_with_query(&question, &String::from(""));
        assert_eq!(bang, None);
    }
}