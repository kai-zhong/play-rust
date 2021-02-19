
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let content = reqwest::get("http://logepi.cn").await?.text().await?;

    println!("{}", content);
    Ok(())
}
