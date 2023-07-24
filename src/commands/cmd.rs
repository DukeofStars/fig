use std::process::Command;

use clap::Args;
use color_eyre::Result;

use crate::repository::Repository;

/// Run a command in the configuration repository directory.
#[derive(Debug, Args)]
pub struct CmdOptions {
    command: String,
}

pub fn cmd(repository: &Repository, options: &CmdOptions) -> Result<()> {
    Command::new(&options.command)
        .current_dir(&repository.dir)
        .status()
        .expect("Failed to spawn child process");

    Ok(())
}
