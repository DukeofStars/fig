use std::process::Command;

use clap::Args;
use color_eyre::{eyre::Context, Result};

use crate::repository::Repository;

/// Run a command in the configuration repository directory.
#[derive(Debug, Args)]
pub struct CmdOptions {
    command: String,
    args: Vec<String>,
}

pub fn cmd(repository: &Repository, options: &CmdOptions) -> Result<()> {
    Command::new(&options.command)
        .args(&options.args)
        .current_dir(&repository.dir)
        .status()
        .context("Failed to spawn child process")?;

    Ok(())
}
