use std::path::PathBuf;

use clap::Args;
use color_eyre::{eyre::bail, Result};
use log::debug;

use crate::repository::Repository;
use crate::template;

#[derive(Debug, Args)]
pub struct InitOptions {
    /// Ignore repositories in the location already
    #[clap(short, long)]
    force: bool,
    /// Location to create fig repository.
    #[clap(short, long)]
    dir: Option<PathBuf>,
}

pub fn init(options: &InitOptions, default_repo_dir: PathBuf) -> Result<Repository> {
    let dir = options.dir.as_ref().unwrap_or_else(|| &default_repo_dir);

    debug!("Creating repository at '{dir}'", dir = dir.display());

    if dir.exists() && !options.force {
        bail!("Already initialised");
    }

    crate::create_dir_all!(&dir)?;

    template::generate(&dir)?;

    let dot_gitignore = "
namespace.fig
    ";
    std::fs::write(dir.join("../../.gitignore"), dot_gitignore)?;

    // Initialise git
    let _ = git2::Repository::init(&dir)?;

    Ok(Repository::open(dir)?)
}
