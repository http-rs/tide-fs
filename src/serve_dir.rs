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

#[cfg(test)]
mod when_serving_directory {
    use super::*;

    use std::{fs::File, io::Write};
    use tempfile::TempDir;

    const TEXT_CONTENT: &str = "hello world";

    /// Initialize a temporary directory with some test files
    fn setup_test_dir() -> TempDir {
        let tmp_dir = TempDir::new().unwrap();

        let mut html_file = File::create(tmp_dir.path().join("index.html")).unwrap();
        write!(html_file, "hello world").unwrap();
        html_file.flush().unwrap();

        let mut text_file = File::create(tmp_dir.path().join("file.txt")).unwrap();
        write!(text_file, "hello world").unwrap();
        text_file.flush().unwrap();

        tmp_dir
    }

    #[async_std::test]
    async fn should_return_file_content() {
        let tmp_dir = setup_test_dir();

        let mut server = tide::new();
        server
            .at("/directory/*path")
            .get(ServeDir::serve(tmp_dir.path(), "path").unwrap());

        let client = surf::Client::with_http_client(server);
        let mut res = client
            .get("http://localhost/directory/index.html")
            .await
            .unwrap();

        assert_eq!(res.body_string().await.unwrap(), TEXT_CONTENT);
    }

    #[async_std::test]
    async fn should_return_404_when_file_not_found() {
        let tmp_dir = setup_test_dir();

        let mut server = tide::new();
        server
            .at("/directory/*path")
            .get(ServeDir::serve(tmp_dir.path(), "path").unwrap());

        let client = surf::Client::with_http_client(server);
        let res = client
            .get("http://localhost/directory/bla.html")
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NotFound);
    }

    #[async_std::test]
    async fn explicit_path_should_override_dir() {
        let tmp_dir = setup_test_dir();

        let mut server = tide::new();
        server
            .at("/*path")
            .get(ServeDir::serve(tmp_dir.path(), "path").unwrap());

        // When defining a specific route that overlaps with the hosted directory
        server.at("index.html").get(|_| async { Ok("stuff") });

        let client = surf::Client::with_http_client(server);
        let mut res = client.get("http://localhost/index.html").await.unwrap();

        // We expect the more explicit route to win
        assert_eq!(res.body_string().await.unwrap(), "stuff");
    }
}
