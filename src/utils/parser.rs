use scraper::Html;
use crate::models::query::Query;
use crate::utils::bang::get_bang_with_query;

pub fn get_html(html: String) -> Html {
    Html::parse_document(&html)
}

pub fn get_query_by_question(question: String) -> Query {
    let mut text = question.trim().split_whitespace().collect::<Vec<&str>>();

    let mut tmp_bang = None;
    let mut bang = None;

    if text.len() > 0 && text[0].starts_with('!') {
        tmp_bang = Some(text[0][1..].to_string());
        text.remove(0);
    }

    let question = text.join(" ");

    if let Some(some_bang) = tmp_bang {
        bang = get_bang_with_query(&question, &some_bang)
    }

    Query {
        text: text.join(" "),
        bang,
    }
}

#[cfg(test)]
mod tests {
    use urlencoding::encode;
    use crate::models::bang::{Bang, BangMode};
    use super::*;

    #[test]
    fn test_detect_bang() {
        let question = "!test aide moi a faire ceci".to_string();
        assert_eq!(get_query_by_question(question), Query {
            text: "aide moi a faire ceci".to_string(),
            bang: None
        });
    }

    #[test]
    fn test_no_bang() {
        assert_eq!(get_query_by_question("test".to_string()), Query {
            text: "test".to_string(),
            bang: None
        });
    }

    #[test]
    fn test_detect_bang_2() {
        let question = "!ia comment faire ceci".to_string();
        let question_encoded = encode("comment faire ceci");
        assert_eq!(
            get_query_by_question(question),
            Query {
                text: "comment faire ceci".to_string(),
                bang: Some(Bang {
                    name: "ia".to_string(),
                    url: format!("https://www.perplexity.ai/search/?q={}", question_encoded),
                    mode: BangMode::Redirect
                })
            });
    }
}