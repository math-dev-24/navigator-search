mod utils;
mod models;
mod search;

use std::time::Instant;

use search::factory::make_engine;
use crate::models::navigator::Navigator;
use crate::utils::parser::get_query_by_question;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let question = "!r Chien".to_string();
    let query = get_query_by_question(question);

    if let None = query.bang {
        let service = make_engine(Navigator::DuckDuckGo);
        let res = service.search(query).await;
        println!("{:?}", res);
    } else {
        let bang = query.bang.unwrap();
        println!("{:?}", bang);
    }

    println!("Temps total : {:?}", start.elapsed());
}