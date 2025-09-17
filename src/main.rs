use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() {
    let namae: Nakami = title().await;
    match namae {
        Ok(e) => println!("{}", e),
        Err(e) => eprintln!("{}", e),
    }
}

type Nakami = Result<String, Box<dyn Error>>;

async fn title() -> Nakami {
    let url = "https://scratch.mit.edu";
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").unwrap();
    let title = document
        .select(&selector)
        .next()
        .map(|e| e.inner_html())
        .unwrap();
    Ok(title)
}
