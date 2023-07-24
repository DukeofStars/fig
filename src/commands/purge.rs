use crate::repository::Repository;
use color_eyre::{eyre::Context, Result};

pub fn purge(repository: &Repository) -> Result<()> {
    std::fs::remove_dir_all(&repository.dir).context("Failed to remove directory")?;

    Ok(())
}
