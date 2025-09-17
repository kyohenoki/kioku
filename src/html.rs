use chrono::Local;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// target/debug/kioqu html [url] [path/filename]
// target/debug/kioqu html https://archlinux.org back/index.html
// default https://example.com back/[timestamp].html

pub async fn html() {
    if let Err(e) = iine().await {
        eprintln!("{}", e);
    }
}

async fn iine() -> Result<(), Box<dyn Error>> {
    if suru("html") {
        let url = aru(2);
        let body = req(&url).await;
        let s: String = body?;
        if let Err(e) = kaku(&s) {
            eprintln!("{}", e);
        }
    }
    Ok(())
}

fn aru(nu: usize) -> String {
    let args: Vec<String> = env::args().collect();
    if let Some(u) = args.get(nu) {
        u.clone()
    } else if nu == 2 {
        "https://example.com".to_string()
    } else if nu == 3 {
        let now = Local::now().timestamp_millis();
        format!("back/{}.html", now)
    } else {
        "none".to_string()
    }
}

fn kaku(naka: &str) -> std::io::Result<()> {
    let path = aru(3);
    let mut file = File::create(path)?;
    file.write_all(naka.as_bytes())?;
    println!("good");
    Ok(())
}

type Nakami = Result<String, Box<dyn Error>>;

pub async fn req(url: &str) -> Nakami {
    let html = reqwest::get(url).await?.text().await?;
    Ok(html)
}

fn suru(name: &str) -> bool {
    let args: Vec<String> = env::args().collect();
    matches!(args.get(1), Some(c) if c == name)
}
