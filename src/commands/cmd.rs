use std::process::Command;

use crate::repository::Repository;
use clap::Args;
use color_eyre::Result;

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
