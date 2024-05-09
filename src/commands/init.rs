use std::path::PathBuf;

use clap::Args;
use color_eyre::eyre::Context;
use color_eyre::Result;
use tracing::warn;

use crate::repository::RepositoryBuilder;

#[derive(Debug, Args)]
pub struct InitOptions {
    /// Ignore repositories in the location already
    #[clap(short, long)]
    force: bool,
    /// Location to create fig repository.
    #[clap(short, long)]
    dir: Option<PathBuf>,
}

pub fn init(mut repo_builder: RepositoryBuilder, options: &InitOptions) -> Result<()> {
    if let Some(dir) = &options.dir {
        repo_builder = RepositoryBuilder::new(dir.clone());
    }

    if options.force && repo_builder.path().exists() {
        warn!("Overriding already initialised repository");
        std::fs::remove_dir_all(repo_builder.path()).context("Failed to remove directory")?;
    }
    let _ = repo_builder.init()?;
    Ok(())
}
