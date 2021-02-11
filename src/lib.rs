//! Tide-fs contains extensions for the Tide web-framework to serve files
//! or whole directories from your file-system. Tide-fs provides a `ServeDir`
//! and a `ServeFile` endpoint. It also provides a convenient extension trait
//! to the `tide::Route` type that allow you to add these endpoints with the
//! `serve_dir` and `serve_file` methods.
//!
//! `ServeFile` serves a single file on a single route;
//! ```rust
//! # use tide::{Request, Result};
//! use tide_fs::prelude::*;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut app = tide::new();
//! app.at("index.html").serve_file("examples/content/index.html")?;
//! # Ok(())
//! # }
//! ```
//!
//!
//! `ServeDir` maps a section of a route to files in a directory
//! ```rust
//! # use tide::{Request, Result};
//! use tide_fs::prelude::*;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut app = tide::new();
//! app.at("static/").serve_dir("examples/content/")?;
//! # Ok(())
//! # }
//! ```

// Turn on warnings for some lints
#![warn(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_qualifications
)]

use std::{io, path::Path};

use prelude::{ServeDir, ServeDirCompiled, ServeFile};
use tide::Route;

pub mod serve_dir;
pub mod serve_dir_compiled;
pub mod serve_file;
pub use include_dir::include_dir;

/// Import everything needed to serve files and directories at the same time
pub mod prelude {
    pub use crate::serve_dir::ServeDir;
    pub use crate::serve_dir_compiled::ServeDirCompiled;
    pub use crate::serve_file::ServeFile;
    pub use crate::TideFsExt;
    pub use include_dir::include_dir;
}

/// Extension methods for serving contents from the filesytem
pub trait TideFsExt {
    /// Serve the contents of a file or directory at this location
    fn serve_fs(&mut self, path: impl AsRef<Path>) -> io::Result<()>;

    fn serve_compiled_dir(
        &mut self,
        dir: include_dir::Dir<'static>,
        index_file: Option<&'static str>,
    );
}

impl<'a, State: Clone + Send + Sync + 'static> TideFsExt for Route<'a, State> {
    fn serve_fs(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();

        if path.is_file() {
            self.get(ServeFile::init(path)?);
        } else {
            self.at("*path").get(ServeDir::init(path, "path")?);
        }

        Ok(())
    }

    fn serve_compiled_dir(
        &mut self,
        dir: include_dir::Dir<'static>,
        index_file: Option<&'static str>,
    ) {
        let serve_dir = ServeDirCompiled::new(dir, "path").with_index_file(index_file);
        self.at("*path").get(serve_dir.clone());

        self.at("").get(serve_dir);
    }
}
