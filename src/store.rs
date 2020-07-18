use md5::Context;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

pub struct Store {
    basedir: PathBuf,
}

impl Store {
    pub fn new(basedir: impl AsRef<Path>) -> Self {
        Store {
            basedir: basedir.as_ref().to_path_buf(),
        }
    }

    pub fn store(&self, name: &str, data: &mut impl Read) -> std::io::Result<[u8; 16]> {
        let path = self.generate_path(name);
        fs::create_dir_all(path.parent().unwrap())?;

        let mut file = File::create(&path)?;

        let mut context = Context::new();
        let mut buf = [0u8; 8 * 1024];

        // copy the file and compute the digest was we go
        loop {
            let len = match data.read(&mut buf) {
                Ok(0) => return Ok(context.compute().0),
                Ok(len) => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };

            let data = &buf[..len];
            context.consume(data);

            file.write_all(data)?;
        }
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
