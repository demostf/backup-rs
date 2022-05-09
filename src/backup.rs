use crate::store::Store;
use crate::Error;
use demostf_client::{ApiClient, Demo, ListOrder, ListParams};
use tracing::{info, instrument};

pub struct Backup {
    client: ApiClient,
    store: Store,
}

impl Backup {
    pub fn new(store: Store) -> Self {
        Backup {
            store,
            client: ApiClient::new(),
        }
    }

    #[instrument(skip_all, fields(demo.id = demo.id, demo.name = name))]
    async fn backup_demo(&self, name: &str, demo: &Demo) -> Result<(), Error> {
        info!("backing up");

        let mut file = self.store.create(name).await?;

        demo.save(&self.client, &mut file).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn backup_page(&self, page: u32) -> Result<usize, Error> {
        let demos = self
            .client
            .list(ListParams::default().with_order(ListOrder::Ascending), page)
            .await?;

        for demo in demos.iter() {
            if !demo.url.is_empty() {
                let name = demo.url.rsplit('/').next().unwrap();
                if !self.store.exists(name) {
                    self.backup_demo(name, demo).await?;
                } else {
                    info!(demo = demo.id, name, "already backed up");
                }
            }
        }

        Ok(demos.len())
    }

    pub async fn backup_from(&self, mut page: u32) -> Result<u32, Error> {
        while self.backup_page(page).await? > 0 {
            page += 1;
        }

        Ok(page)
    }
}
