use async_std::path::PathBuf as AsyncPathBuf;
use log;
use std::ffi::OsStr;
use std::io;
use std::path::Path;
use tide::Body;
use tide::Response;
use tide::{utils::async_trait, Endpoint};
use tide::{Request, Result, StatusCode};

/// Endpoint for serving a directory
#[derive(Clone, Debug, PartialEq)]
pub struct ServeDir {
    dir_path: AsyncPathBuf,
    pattern: String,
}

impl ServeDir {
    /// Construct an endpoint for serving a directory. dir_path is the path of the directory to serve
    /// pattern is the name of the pattern from the request.
    pub fn serve(dir_path: impl AsRef<Path>, pattern: &str) -> io::Result<Self> {
        Ok(Self {
            dir_path: AsyncPathBuf::from(dir_path.as_ref().to_owned().canonicalize()?),
            pattern: pattern.to_string(),
        })
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeDir {
    async fn call(&self, req: Request<State>) -> Result {
        let path = req.param(&self.pattern)?.trim_start_matches('/');

        let mut file_path = self.dir_path.clone();
        for p in Path::new(path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                file_path.pop();
            } else {
                file_path.push(&p);
            }
        }

        log::info!("Requested file: {:?}", file_path);

        if !file_path.starts_with(&self.dir_path) {
            log::warn!("Unauthorized attempt to read: {:?}", file_path);
            Ok(Response::new(StatusCode::Forbidden))
        } else {
            match Body::from_file(&file_path).await {
                Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    Ok(Response::new(StatusCode::NotFound))
                }
                Err(e) => Err(e.into()),
            }
        }
    }
}

/// Endpoint for serving files, file_path is the path to the file to serve
#[derive(Clone, Debug, PartialEq)]
pub struct ServeFile {
    file_path: AsyncPathBuf,
}

impl ServeFile {
    pub fn serve(file_path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            file_path: AsyncPathBuf::from(file_path.as_ref().to_owned().canonicalize()?),
        })
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for ServeFile {
    async fn call(&self, _req: Request<State>) -> Result {
        match Body::from_file(&self.file_path).await {
            Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Ok(Response::new(StatusCode::NotFound))
            }
            Err(e) => Err(e.into()),
        }
    }
}
