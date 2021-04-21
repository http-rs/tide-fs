//! Endpoint for serving directory contents from a directory compiled into the binary

use include_dir::{Dir, DirEntry, File};
use std::ffi::OsStr;
use tide::http::{content::ContentType, mime::Mime};
use tide::{utils::async_trait, Endpoint, Request, Response, Result};

/// Endpoint for serving directories compiled into the binary
#[derive(Debug, Clone)]
pub struct ServeDirCompiled {
    dir: Dir<'static>,
    index_file: Option<&'static str>,
    pattern: &'static str,
}

impl ServeDirCompiled {
    /// Construct a new `ServeDirCompiled` struct
    pub fn new(dir: Dir<'static>, pattern: &'static str) -> Self {
        Self {
            pattern,
            dir,
            index_file: None,
        }
    }

    /// Set an index file
    pub fn with_index_file(mut self, file: Option<&'static str>) -> Self {
        self.index_file = file;
        self
    }

    fn serve_file(&self, file: File) -> Response {
        let mut res = Response::new(200);
        if let Some(mime) = file
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .and_then(Mime::from_extension)
        {
            ContentType::new(mime).apply(&mut res);
        }
        res.set_body(file.contents());
        res
    }

    fn get_item(&self, path: &str) -> Option<DirEntry> {
        if path == "" {
            Some(DirEntry::Dir(self.dir))
        } else if let Some(dir) = self.dir.get_dir(path) {
            Some(DirEntry::Dir(dir))
        } else if let Some(file) = self.dir.get_file(path) {
            Some(DirEntry::File(file))
        } else {
            None
        }
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeDirCompiled {
    async fn call(&self, req: Request<State>) -> Result {
        let entry = self.get_item(
            req.param(self.pattern)
                .unwrap_or("")
                .trim_start_matches('/'),
        );

        let response = match (entry, self.index_file) {
            (None, _) => None,
            (Some(DirEntry::File(file)), _) => Some(self.serve_file(file)),
            (Some(DirEntry::Dir(_)), None) => None,
            (Some(DirEntry::Dir(dir)), Some(index_file)) => {
                if let Some(file) = dir.get_file(index_file) {
                    Some(self.serve_file(file))
                } else {
                    None
                }
            }
        };

        Ok(response.unwrap_or_else(|| Response::new(404)))
    }
}
