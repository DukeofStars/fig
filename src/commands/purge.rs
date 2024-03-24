use color_eyre::{eyre::Context, Result};

use crate::repository::RepositoryBuilder;

pub fn purge(repo_builder: RepositoryBuilder) -> Result<()> {
    let repository = repo_builder
        .open()
        .context("Cannot purge repository if it doesn't exist!")?;

    std::fs::remove_dir_all(repository.path()).context("Failed to remove directory")?;

    Ok(())
}
