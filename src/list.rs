use std::path::Path;

use clap::Args;
use miette::Result;

use crate::{repository::Repository, strip_namespace};

#[derive(Args)]
pub struct ListOptions {
    #[clap(short, long)]
    tree: bool,
}

pub fn list(repository: &Repository, options: ListOptions) -> Result<()> {
    for (name, path) in repository.namespaces()? {
        let dir = repository.dir.join(&name);
        if options.tree {
            println!("{}: {}", name, path.display());
        }
        recurse_dir(&options, &dir);
    }

    Ok(())
}

fn recurse_dir(options: &ListOptions, path: impl AsRef<Path>) {
    let path = path.as_ref();
    let entries = path.read_dir().expect("Failed to read directory");
    for entry in entries {
        let entry = entry.expect("Invalid entry");
        // Don't include namespace files
        if entry.path().file_name().unwrap() != "namespace.fig" && entry.path().is_file() {
            let display_path =
                strip_namespace(&path, entry.path()).expect("Failed to strip namespace");
            println!(
                "{}{}",
                if options.tree { "\t" } else { "" },
                path.join(display_path).display()
            );
        } else if entry.path().is_dir() {
            recurse_dir(options, entry.path());
        }
    }
}
