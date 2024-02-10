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

            if !options.namespace.is_empty() && !options.namespace.contains(&name.to_string()) {
                continue;
            }

            if options.tree {
                println!(
                    "{:12}: {}",
                    name,
                    match ns.targets.len() {
                        1 => {
                            ns.targets.get(0).unwrap().display().to_string()
                        }
                        2.. => {
                            ns.targets
                                .iter()
                                .map(|p| p.display().to_string())
                                .collect::<Vec<String>>()
                                .join(&format!(
                                    "{}\n",
                                    " ".repeat(14 /* Align the targets with each other */)
                                ))
                        }
                        _ => panic!(),
                    }
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
