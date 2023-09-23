use clap::Args;
use color_eyre::eyre::Context;
use color_eyre::Result;

use fig::repository::RepositoryBuilder;

use crate::info::Info;

/// Get information about the configuration repository.
#[derive(Debug, Args)]
pub struct InfoOptions {
    #[clap(long)]
    repo_dir: bool,
    #[clap(long)]
    log_dir: bool,
    #[clap(long)]
    json: bool,
}

pub fn info(repo_builder: RepositoryBuilder, options: &InfoOptions) -> Result<()> {
    let repository = repo_builder.open()?;

    let info = Info::gather(&repository)?;

    if options.json {
        let json = serde_json::to_string_pretty(&info).context("Failed to serialize Info")?;
        println!("{json}");
        return Ok(());
    }

    if options.repo_dir {
        println!("repository dir: {}", info.repository_dir.display())
    }
    if options.log_dir {
        println!("{}", info.log_dir.display())
    }

    Ok(())
}
