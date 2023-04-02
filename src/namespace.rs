use std::{fs, path::PathBuf};

use clap::{Args, Subcommand};
use miette::*;
use owo_colors::OwoColorize;

use crate::repository::Repository;

#[derive(Args)]
pub struct NamespaceOptions {
    #[clap(subcommand)]
    subcommand: Command,
}
#[derive(Subcommand)]
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

pub fn namespace_cli(repository: &Repository, options: NamespaceOptions) -> Result<()> {
    match options.subcommand {
        Command::List { json } => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&repository.namespaces()?).unwrap()
                )
            } else {
                for (name, path) in repository.namespaces()? {
                    println!("{:12}: {}", name.blue(), path.display().bright_blue());
                }
            }
            Ok(())
        }
        Command::Add { name, path } => {
            let dir = repository.dir.join(&name);
            let namespace_file = dir.join("namespace.fig");

            fs::create_dir(&dir)
                .into_diagnostic()
                .wrap_err("Failed to create namespace directory")?;

            let path = path.canonicalize().unwrap();
            let path = path.to_str().unwrap().trim_start_matches("\\\\?\\");

            fs::write(&namespace_file, &path)
                .into_diagnostic()
                .wrap_err("Failed to write to namespace file")?;

            println!("Added namespace {}: {}", name.blue(), path.bright_blue());

            Ok(())
        }
        Command::Remove { name } => {
            let namespaces = repository.namespaces()?;
            ensure!(
                namespaces.contains_key(&name),
                "The namespace {name} does not exist"
            );

            let namespace_root = repository.dir.join(&name);

            // Get number of files inside namespace
            let count = {
                let files = crate::list::get_all_files(repository)?;
                let files_in_ns = &files
                    .iter()
                    .find(|(name_, _files)| name_ == &name)
                    .unwrap()
                    .1;

                files_in_ns.iter().count()
            };

            if count > 0 {
                print!("Are you sure you want to delete {name}? It contains {count} files. [y/N] ");
                let mut buf = String::new();
                std::io::stdin()
                    .read_line(&mut buf)
                    .expect("Failed to read from stdin");
                let buf = buf.trim().to_lowercase();
                if buf == "y" {
                } else {
                    return Ok(());
                }
            }

            fs::remove_dir_all(&namespace_root)
                .into_diagnostic()
                .wrap_err("Failed to remove namespace directory")?;

            Ok(())
        }
    }
}
