use tide_fs::prelude::*;

fn main() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("static/css/").serve_dir("static_content/css/")?;
    Ok(())
}
