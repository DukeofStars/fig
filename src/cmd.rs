use std::process::Command;

use clap::Args;
use miette::Result;

use crate::repository::Repository;

#[derive(Args)]
pub struct CmdOptions {
    command: String,
}

pub fn cmd(repository: &Repository, options: CmdOptions) -> Result<()> {
    Command::new(options.command)
        .current_dir(&repository.dir)
        .status()
        .expect("Failed to spawn child process");

    Ok(())
}
