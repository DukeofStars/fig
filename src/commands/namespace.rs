use std::path::PathBuf;

use clap::{Args, Subcommand};
use color_eyre::eyre::{Context, ensure};
use color_eyre::Result;
use owo_colors::OwoColorize;

use crate::repository::Repository;

#[derive(Debug, Args)]
pub struct NamespaceOptions {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    List {
        #[clap(long)]
        json: bool,
    },
    Add {
        name: String,
        #[clap(default_value = ".")]
        path: PathBuf,
    },
    Remove {
        name: String,
    },
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
            let dir = repository.dir.join(&name);
            let namespace_file = dir.join("namespace.fig");

            crate::create_dir_all!(&dir).context("Failed to create namespace directory")?;

            let path = path.canonicalize().unwrap();
            let path = path.to_str().unwrap().trim_start_matches("\\\\?\\");

            std::fs::write(namespace_file, path).context("Failed to write to namespace file")?;

            println!("Added namespace {}: {}", name.blue(), path.bright_blue());

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

            let namespace_root = repository.dir.join(&name);

            print!("Are you sure you want to delete {name}? [y/N] ");
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin");
            let buf = buf.trim().to_lowercase();
            if buf == "y" {} else {
                return Ok(());
            }

            crate::remove_dir_all!(&namespace_root)
                .context("Failed to remove namespace directory")?;

            Ok(())
        }
    }
}
