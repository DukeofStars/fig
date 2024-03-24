use std::process::Command;

use clap::Args;
use color_eyre::{eyre::Context, Result};

use crate::repository::RepositoryBuilder;

#[derive(Debug, Args)]
pub struct CmdOptions {
    command: String,
    args: Vec<String>,
}

pub fn cmd(repo_builder: RepositoryBuilder, options: &CmdOptions) -> Result<()> {
    let repository = repo_builder.open()?;
    Command::new(&options.command)
        .args(&options.args)
        .current_dir(repository.path())
        .status()
        .context("Failed to spawn child process")?;

    Ok(())
}
