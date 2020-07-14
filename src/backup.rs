use crate::api::{list_demos, ListOrder, ListParams};
use crate::store::Store;
use crate::Error;
use md5::Digest;

pub struct Backup {
    store: Store,
}

impl Backup {
    pub fn new(store: Store) -> Self {
        Backup { store }
    }

    fn backup_demo(&self, name: &str, url: &str, hash: Digest) -> Result<(), Error> {
        let resp = ureq::get(url).call();

        let digest = self.store.store(name, &mut resp.into_reader())?;

        if digest == hash || digest == Digest([0; 16]) {
            Ok(())
        } else {
            let _ = self.store.remove(name);
            Err(Error::DigestMismatch {
                expected: hash,
                got: digest,
            })
        }
    }

    fn backup_page(&self, page: u32) -> Result<usize, Error> {
        let demos = list_demos(ListParams::default().with_order(ListOrder::Ascending), page)?;

        for demo in demos.iter() {
            if demo.url != "" {
                let name = demo.url.rsplit('/').next().unwrap();
                println!("{} {}", demo.id, name);
                if !self.store.exists(name) {
                    self.backup_demo(name, &demo.url, demo.hash)?;
                }
            }
        }

        Ok(demos.len())
    }

    pub fn backup_from(&self, mut page: u32) -> Result<u32, Error> {
        while self.backup_page(page)? > 0 {
            page += 1;
        }

        Ok(page)
    }
}
