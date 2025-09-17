use scraper::{Html, Selector};
use std::env;
use std::error::Error;

// target/debug/kioqu title [url]
// target/debug/kioqu title https://archive.md
// default https://archive.org

pub async fn title() {
    let args: Vec<String> = env::args().collect();
    if matches!(args.get(1), Some(c) if c == "title") {
        let mut url = "https://archive.org".to_string();
        if let Some(u) = args.get(2) {
            url = u.clone();
        }
        let namae: Nakami = req(&url).await;
        match namae {
            Ok(e) => println!("{}", e),
            Err(e) => eprintln!("{}", e),
        }
    }
}

type Nakami = Result<String, Box<dyn Error>>;

async fn req(url: &str) -> Nakami {
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").unwrap();
    let title = document.select(&selector).next().map(|e| e.inner_html()).unwrap();
    Ok(title)
}
