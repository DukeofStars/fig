use std::path::PathBuf;

use clap::Args;
use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use log::debug;

use crate::{repository::Repository, template};

/// Initialise a configuration repository.
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
    let dir = options.dir.as_ref().unwrap_or(&default_repo_dir);

    debug!("Creating repository at '{dir}'", dir = dir.display());

    if dir.exists() && !options.force {
        bail!("Already initialised");
    }

    crate::create_dir_all!(dir)
        .context(format!("Failed to create directory '{}'", dir.display()))?;

    template::generate(dir)?;

    let dot_gitignore = "
namespace.fig
    ";
    let dot_gitignore_path = dir.join("../../.gitignore");
    std::fs::write(&dot_gitignore_path, dot_gitignore).context(format!(
        "Failed to write to {}",
        dot_gitignore_path.display()
    ))?;

    // Initialise git
    let _ = git2::Repository::init(dir)?;

    Ok(Repository::open(dir)?)
}
