use std::path::PathBuf;

use clap::{Args, Subcommand};
use color_eyre::eyre::{ensure, Context};
use color_eyre::Result;
use owo_colors::OwoColorize;

use crate::repository::Repository;

/// Manage your configuration namespaces
#[derive(Debug, Args)]
pub struct NamespaceOptions {
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List your configuration namespaces.
    List {
        #[clap(long)]
        json: bool,
    },
    /// Create a new namespace.
    Add {
        name: String,
        #[clap(default_value = ".")]
        path: PathBuf,
    },
    /// Remove a namespace.
    Remove { name: String },
}

pub fn namespace_cli(repository: &Repository, options: &NamespaceOptions) -> Result<()> {
    match &options.subcommand {
        Command::List { json } => {
            if *json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&repository.namespaces()?).unwrap()
                )
            } else {
                for namespace in repository.namespaces()? {
                    println!(
                        "{:12}: {}",
                        namespace
                            .location
                            .file_name()
                            .expect("No file name?")
                            .to_str()
                            .unwrap()
                            .blue(),
                        namespace.target.display().bright_blue()
                    );
                }
            }
            Ok(())
        }
        Command::Add { name, path } => {
            let dir = repository.dir.join(name);
            let namespace_file = dir.join("namespace.fig");

            crate::create_dir_all!(&dir).context("Failed to create namespace directory")?;

            let path = path.canonicalize()?;

            std::fs::write(namespace_file, path.display().to_string())
                .context("Failed to write to namespace file")?;

            println!(
                "Added namespace {}: {}",
                name.blue(),
                path.display().bright_blue()
            );

            Ok(())
        }
        Command::Remove { name } => {
            let namespaces = repository.namespaces()?;
            ensure!(
                namespaces.iter().any(|ns| ns
                    .location
                    .file_name()
                    .expect("No file name?")
                    .to_str()
                    .unwrap()
                    == name),
                "The namespace {name} does not exist"
            );

            let namespace_root = repository.dir.join(name);

            print!("Are you sure you want to delete {name}? [y/N] ");
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin");
            let buf = buf.trim().to_lowercase();
            if buf != "y" {
                return Ok(());
            }

            crate::remove_dir_all!(&namespace_root)
                .context("Failed to remove namespace directory")?;

            Ok(())
        }
    }
}
