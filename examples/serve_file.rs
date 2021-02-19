use tide_fs::prelude::*;

fn main() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("index.html").serve_file("files/index.html")?;
    Ok(())
}
