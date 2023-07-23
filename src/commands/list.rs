use std::{collections::HashMap, path::PathBuf};

use crate::repository::Repository;
use clap::Args;
use color_eyre::eyre::Context;
use color_eyre::Result;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
/// Print all files that are tracked by fig.
pub struct ListOptions {
    #[clap(short, long)]
    tree: bool,
    #[clap(short, long, alias = "ns")]
    namespace: Vec<String>,
    #[clap(long)]
    json: bool,
}

pub fn list(repository: &Repository, options: &ListOptions) -> Result<()> {
    if options.json {
        let mut files: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for ns in repository.namespaces()? {
            let ns_files = ns.files()?;
            let name = ns
                .location
                .file_name()
                .expect("Failed to get file name")
                .to_str()
                .expect("Failed to convert OsString to &str")
                .to_string();
            files.insert(name, ns_files);
        }

        let json = serde_json::to_string_pretty(&files).expect("Failed to serialize files");
        println!("{json}");

        return Ok(());
    }

    for ns in repository
        .namespaces()
        .context("Failed to collect namespaces")?
    {
        let name = ns
            .location
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert OsString to &str");

        if !options.namespace.is_empty() && !options.namespace.contains(&name.to_string()) {
            continue;
        }

        if options.tree {
            println!(
                "{namespace:12}: {path}",
                namespace = name.blue(),
                path = ns.target.display().bright_blue()
            );
        }
        let files = ns.files()?;
        for file in files {
            println!(
                "{}{path}",
                if options.tree {
                    " ".repeat(12)
                } else {
                    "".to_string()
                },
                path = file.display()
            );
        }
    }

    Ok(())
}
