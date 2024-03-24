use std::{collections::HashMap, path::PathBuf};

use clap::Args;
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

use crate::repository::RepositoryBuilder;

#[derive(Debug, Args)]
pub struct ListOptions {
    /// Display files in their respective namespace
    #[clap(short, long)]
    pretty: bool,
    /// Only show files from certain namespace
    #[clap(short, long)]
    filter: Vec<String>,
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
            let name = path.file_name().unwrap().to_str().unwrap();
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

            if !options.filter.is_empty() && !options.filter.contains(&name.to_string()) {
                continue;
            }

            if options.pretty {
                println!("-- {}", name,);
            }
            let files = ns.files()?;
            for file in files {
                println!("{path}", path = file.display());
            }
        }

        Ok(())
    }
}
