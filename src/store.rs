use crate::Error;
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use md5::Context;
use std::fs;
use std::fs::{File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tokio::pin;

pub struct Store {
    basedir: PathBuf,
}

impl Store {
    pub fn new(basedir: impl AsRef<Path>) -> Self {
        Store {
            basedir: basedir.as_ref().to_path_buf(),
        }
    }

    pub async fn store(
        &self,
        name: &str,
        data: impl Stream<Item = Result<Bytes, demostf_client::Error>>,
    ) -> Result<[u8; 16], Error> {
        let path = self.generate_path(name);
        fs::create_dir_all(path.parent().unwrap())?;

        let mut file = File::create(&path)?;

        let mut context = Context::new();

        pin!(data);
        // copy the file and compute the digest was we go
        while let Some(chunk) = data.next().await {
            let chunk = chunk?;
            context.consume(&chunk);
            file.write_all(&chunk)?;
        }
        file.set_permissions(Permissions::from_mode(0o644))?;

        Ok(context.compute().0)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.generate_path(name).is_file()
    }

    pub fn remove(&self, name: &str) -> std::io::Result<()> {
        fs::remove_file(self.generate_path(name))
    }

    fn generate_path(&self, name: &str) -> PathBuf {
        let mut path = self.basedir.clone();
        path.push(&name[0..2]);
        path.push(&name[2..4]);
        path.push(name);
        path
    }
}
