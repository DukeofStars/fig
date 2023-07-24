use clap::Args;
use color_eyre::Result;

use crate::info::Info;
use crate::repository::Repository;

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

pub fn info(repo: &Repository, options: &InfoOptions) -> Result<()> {
    let info = Info::gather(repo)?;

    if options.json {
        let json = serde_json::to_string_pretty(&info).expect("Failed to serialize Info");
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
