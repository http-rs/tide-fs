//! Tide-fs contains extensions for the Tide web-framework to serve files
//! or whole directories from your file-system. Tide-fs provides a `ServeDir`
//! and a `ServeFile` endpoint. It also provides a convenient extension trait
//! to the `tide::Route` type that allow you to add these endpoints with the 
//! `serve_dir` and `serve_file` methods.
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
//! app.at("index.html").serve_file("files/index.html")?;
//! # Ok(())
//! # }
//! ```
//!
//!
//! `ServeDir` maps a section of a route to files in a directory
//! ```rust
//! # use tide::{Request, Result};
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! use tide_fs::prelude::*;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut app = tide::new();
//! app.at("static/css/").serve_dir("static_content/css/")?;
//! # Ok(())
//! # }
//! ```
//! The ServeDir endpoint requires you to define a route with a parameter
//! the value of which is then mapped to files inside the directory that is served.

use std::{io, path::Path};

use prelude::{ServeDir, ServeFile};
use tide::Route;

pub mod serve_dir;
pub mod serve_file;

pub mod prelude {
    pub use crate::serve_dir::ServeDir;
    pub use crate::serve_file::ServeFile;
    pub use crate::TideFsExt;
}

pub trait TideFsExt {
    fn serve_file(&mut self, file: impl AsRef<Path>) -> io::Result<()>;
    fn serve_dir(&mut self, dir: impl AsRef<Path>) -> io::Result<()>;
}

impl<'a, State: Clone + Send + Sync + 'static> TideFsExt for Route<'a, State> {
    fn serve_file(&mut self, file: impl AsRef<Path>) -> io::Result<()> {
        self.get(ServeFile::init(file)?);
        Ok(())
    }

    fn serve_dir(&mut self, dir: impl AsRef<Path>) -> io::Result<()> {
        self.at("*path").get(ServeDir::init(dir, "path")?);
        Ok(())
    }
}
