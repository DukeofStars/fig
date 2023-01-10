use clap::Args;
use miette::Result;

use crate::{namespace::strip_namespace, repository::Repository};

#[derive(Args)]
pub struct ListOptions {}

pub fn list(repository: &Repository, _options: ListOptions) -> Result<()> {
    for (name, path) in repository.namespaces()? {
        let dir = repository.dir.join(name);
        let entries = dir.read_dir().expect("Failed to read directory");
        for entry in entries {
            let entry = entry.expect("Invalid entry");
            // Don't include namespace files
            if entry.path().file_name().unwrap() != "namespace.fig" {
                let display_path =
                    strip_namespace(&dir, entry.path()).expect("Failed to strip namespace");
                println!("\t{}", path.join(display_path).display());
            }
        }
    }

    Ok(())
}
