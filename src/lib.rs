//! Tide-fs contains extensions for the Tide web-framework to serve files
//! or whole directories from your file-system. Tide-fs provides a `ServeDir`
//! and a `ServeFile` endpoint.
//!
//! `ServeFile` serves a single file on a single route;
//! ```rust
//! # use tide::{Request, Result};
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! use tide_fs::prelude::*;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut app = tide::new();
//! app.at("index.html").get(ServeFile::serve("files/index.html")?);
//! # Ok(())
//! # }
//! ```
//!
//!
//! `ServeDir` maps a section of a route to a directory, making all files
//! inside that directory available on their own Url;
//! ```rust
//! # use tide::{Request, Result};
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! use tide_fs::prelude::*;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut app = tide::new();
//! app.at("static/css/*path").get(ServeDir::serve("static_content/css/", "path")?);
//! # Ok(())
//! # }
//! ```
//! The ServeDir endpoint requires you to define a route with a parameter
//! the value of which is then mapped to files inside the directory that is served.

pub mod serve_dir;
pub mod serve_file;

pub mod prelude {
    pub use crate::serve_dir::ServeDir;
    pub use crate::serve_file::ServeFile;
}
