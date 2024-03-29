mod backup;
mod store;

use crate::backup::Backup;
use crate::store::Store;
use main_error::MainError;
use std::cmp::max;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request failed: {0}")]
    Request(#[from] std::io::Error),
    #[error(transparent)]
    Api(#[from] demostf_client::Error),
    #[error("MD5 digest mismatch for downloaded demo, expected {expected:?}, received {got:?}")]
    DigestMismatch { expected: [u8; 16], got: [u8; 16] },
    #[error("Backup timed out")]
    Timeout,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    tracing_subscriber::fmt::init();

    let mut args: HashMap<_, _> = dotenvy::vars().collect();
    let store = Store::new(args.get("STORAGE_ROOT").expect("no STORAGE_ROOT set"));
    let state_path = PathBuf::from(args.remove("STATE_FILE").expect("no STATE_FILE set"));
    let backup = Backup::new(store);

    let last_page = if state_path.is_file() {
        max(
            std::fs::read_to_string(&state_path)?
                .trim()
                .parse::<u32>()?
                - 1,
            1,
        )
    } else {
        1u32
    };
    info!(last_page, "starting backup");

    let current_page = backup.backup_from(last_page).await?;

    std::fs::write(&state_path, format!("{}", current_page))?;

    Ok(())
}
