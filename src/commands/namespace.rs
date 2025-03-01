use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Subcommand};
use color_eyre::eyre::{ensure, Context};
use color_eyre::Result;

use crate::repository::RepositoryBuilder;

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
    #[clap(alias = "rm")]
    Remove { name: String },
}

pub fn namespace_cli(repo_builder: RepositoryBuilder, options: &NamespaceOptions) -> Result<()> {
    let repository = repo_builder.open()?;

    match &options.subcommand {
        Command::List { json } => {
            if *json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&repository.namespaces()?).unwrap()
                );
            } else {
                for namespace in repository.namespaces()? {
                    println!(
                        "{:12}: {}",
                        namespace
                            .location
                            .file_name()
                            .expect("No file name?")
                            .to_str()
                            .unwrap(),
                        match namespace.targets.len() {
                            1 => {
                                namespace.targets.get(0).unwrap().display().to_string()
                            }
                            2.. => {
                                namespace
                                    .targets
                                    .iter()
                                    .map(|p| p.display().to_string())
                                    .collect::<Vec<String>>()
                                    .join(&format!(
                                        "{}\n",
                                        " ".repeat(14 /* Align the targets with each other */)
                                    ))
                            }
                            // TODO: Namespaces with no listed targets will completely break the program and it is not at all reported to the user.
                            _ => panic!(),
                        }
                    );
                }
            }
            Ok(())
        }
        Command::Add { name, path } => {
            let dir = repository.path().join(name);
            let namespace_file = dir.join("namespace.fig");

            crate::create_dir_all!(&dir).context("Failed to create namespace directory")?;

            let path = path.canonicalize()?;

            std::fs::write(namespace_file, path.display().to_string())
                .context("Failed to write to namespace file")?;

            tracing::info!(%name, path = %path.display(), "Created namespace");
            println!("Added namespace {}: {}", name, path.display());

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

            tracing::info!("Removing namespace: {}", name);

            let namespace_root = repository.path().join(name);

            print!("Are you sure you want to delete {name}? [y/N] ");
            std::io::stdout().flush()?;
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin");
            let buf = buf.trim().to_lowercase();
            if buf != "y" && buf != "yes" {
                return Ok(());
            }

            crate::remove_dir_all!(&namespace_root)
                .context("Failed to remove namespace directory")?;
            tracing::info!(%name, path = %namespace_root.display(), "Removed namespace");
            println!("Removed namespace {}: {}", name, namespace_root.display());

            Ok(())
        }
    }
}
