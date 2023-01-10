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
        let entries = dir.read_dir().expect("Failed to read directory");
        if options.tree {
            println!("{}: {}", name, path.display());
        }
        for entry in entries {
            let entry = entry.expect("Invalid entry");
            // Don't include namespace files
            if entry.path().file_name().unwrap() != "namespace.fig" {
                let display_path =
                    strip_namespace(&dir, entry.path()).expect("Failed to strip namespace");
                println!(
                    "{}{}",
                    if options.tree { "\t" } else { "" },
                    path.join(display_path).display()
                );
            }
        }
    }

    Ok(())
}
