mod utils;
mod models;
mod search;

use std::time::Instant;

use search::factory::make_engine;
use crate::models::item::Item;
use crate::models::navigator::Navigator;
use crate::utils::parser::get_query_by_question;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let question = "!yt Apprendre rust".to_string();
    let query = get_query_by_question(question);

    let service = make_engine(Navigator::DuckDuckGo);

    let res = service.search(&query).await.unwrap();

    println!("{:?}", res);
    println!("Found : {}", res.len());
    println!("Temps total : {:?}", start.elapsed());
}