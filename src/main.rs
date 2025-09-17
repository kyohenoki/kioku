mod html;
mod title;

#[tokio::main]
async fn main() {
    title::title().await;
    html::html().await;
}
