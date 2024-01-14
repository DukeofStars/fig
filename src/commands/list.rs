use std::{collections::HashMap, path::PathBuf};

use clap::Args;
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

use crate::repository::RepositoryBuilder;

#[derive(Debug, Args)]
pub struct ListOptions {
    #[clap(short, long)]
    tree: bool,
    #[clap(short, long, alias = "ns")]
    namespace: Vec<String>,
    #[clap(long)]
    json: bool,
}

pub fn list(repo_builder: RepositoryBuilder, options: &ListOptions) -> Result<()> {
    let repository = repo_builder.open()?;

    if options.json {
        let mut files: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for ns in repository.namespaces()? {
            let ns_files = ns.files()?;
            let path = ns.location.canonicalize()?;
            let name = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or(eyre!("Failed to convert OsStr to &str"))?;
            files.insert(name.to_string(), ns_files);
        }

        let json = serde_json::to_string_pretty(&files).expect("Failed to serialize files");
        println!("{json}");

        Ok(())
    } else {
        for ns in repository
            .namespaces()
            .context("Failed to collect namespaces")?
        {
            let path = ns.location.canonicalize()?;
            let name = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or(eyre!("Failed to convert OsStr to &str"))?;

            if !options.namespace.is_empty() && !options.namespace.contains(&name.to_string()) {
                continue;
            }

            if options.tree {
                println!(
                    "{namespace:12}: {path}",
                    namespace = name,
                    path = ns.target.display()
                );
            }
            let files = ns.files()?;
            for file in files {
                println!(
                    "{}{path}",
                    if options.tree {
                        " ".repeat(12)
                    } else {
                        String::new()
                    },
                    path = file.display()
                );
            }
        }

        Ok(())
    }
}
