use std::error::Error;
use reqwest;
use scraper::{Html, Selector};

// URLのタイトルを取得してみる

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://archive.org";
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").unwrap();
    let title = document.select(&selector).next().map(|e| e.inner_html()).unwrap();
    println!("{}", title);
    Ok(())
}
