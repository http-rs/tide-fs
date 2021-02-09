use async_std::path::PathBuf as AsyncPathBuf;
use std::io;
use std::path::Path;
use tide::Body;
use tide::Response;
use tide::{utils::async_trait, Endpoint};
use tide::{Request, Result, StatusCode};

/// Endpoint for serving files, file_path is the path to the file to serve
#[derive(Clone, Debug, PartialEq)]
pub struct ServeFile {
    file_path: AsyncPathBuf,
}

impl ServeFile {
    pub fn init(file_path: impl AsRef<Path>) -> io::Result<Self> {
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

#[cfg(test)]
mod when_serving_file {
    use super::*;

    use std::{fs::File, io::Write};
    use tempfile::TempDir;

    const TEXT_CONTENT: &str = "hello world";

    /// Initialize a temporary directory with some test files
    fn setup_test_file() -> TempDir {
        let tmp_dir = TempDir::new().unwrap();

        let mut text_file = File::create(tmp_dir.path().join("file.txt")).unwrap();
        write!(text_file, "hello world").unwrap();
        text_file.flush().unwrap();

        tmp_dir
    }

    #[async_std::test]
    async fn should_return_file_contents() {
        let tmp_dir = setup_test_file();

        let mut server = tide::new();
        server
            .at("/file")
            .get(ServeFile::init(tmp_dir.path().join("file.txt")).unwrap());

        let client = surf::Client::with_http_client(server);
        let mut res = client.get("http://localhost/file").await.unwrap();

        assert_eq!(res.body_string().await.unwrap(), TEXT_CONTENT);
    }
}
