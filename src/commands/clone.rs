use clap::Args;
use color_eyre::Result;
use url::Url;

use fig::repository::RepositoryBuilder;

#[derive(Debug, Args)]
pub struct CloneOptions {
    url: Url,
}

pub fn clone(repo_builder: RepositoryBuilder, options: &CloneOptions) -> Result<()> {
    RepositoryBuilder::clone(&options.url)?;

    Ok(())
}
