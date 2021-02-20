
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let content = reqwest::get("https://rust-lang.org").await?.text().await?;

    println!("{}", content);
    Ok(())
}
