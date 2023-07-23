use std::path::PathBuf;

use clap::Parser;
use fig::{repository::Repository, template};
use log::debug;
use miette::{bail, IntoDiagnostic};

#[derive(Parser)]
pub struct Options {
    /// Ignore repositories in the location already
    #[clap(short, long)]
    force: bool,
    /// Location to create fig repository.
    #[clap(short, long)]
    dir: Option<PathBuf>,
}

fn main() -> miette::Result<()> {
    let options = Options::parse();
    let dir = options.dir.unwrap_or_else(Repository::dir);

    debug!("Creating repository at '{dir}'", dir = dir.display());

    if dir.exists() && !options.force {
        bail!("Already initialised");
    }

    std::fs::create_dir_all(&dir).into_diagnostic()?;

    template::generate(&dir)?;

    let dot_gitignore = "
namespace.fig
    ";
    std::fs::write(dir.join(".gitignore"), dot_gitignore).into_diagnostic()?;

    // Initialise git
    let _ = git2::Repository::init(&dir).into_diagnostic()?;

    Ok(())
}
