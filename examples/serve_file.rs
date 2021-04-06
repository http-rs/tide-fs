use tide_fs::prelude::*;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("index.html")
        .serve_fs("examples/content/index.html")?;
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
