use tide_fs::prelude::*;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/")
        .serve_compiled_dir(include_dir!("examples/static"), Some("index.html"));
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
