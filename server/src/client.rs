use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    pin::Pin,
};
use tokio::{fs::File, io::AsyncRead};

pub struct Client {
    directory: PathBuf,
}

impl Client {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    pub async fn put_file<'a>(
        &'a self,
        dir: PathBuf,
        filename: String,
        mut reader: Pin<Box<dyn AsyncRead + Send + Sync + 'a>>,
    ) -> Result<()> {
        let path = Path::new(&self.directory).join(dir).join(filename);
        let mut file = File::create(path).await?;

        tokio::io::copy(&mut reader, &mut file).await?;
        Ok(())
    }

    pub async fn get_file(
        &self,
        filename: PathBuf,
    ) -> Result<Pin<Box<dyn AsyncRead + Send + Sync>>> {
        let path = Path::new(&self.directory).join(filename);

        Ok(Box::pin(tokio::fs::File::open(path).await?))
    }
}
