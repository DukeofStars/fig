use std::path::Path;

use clap::Args;
use miette::{IntoDiagnostic, Result};

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
        recurse_dir(&options, &path, &dir).into_diagnostic()?;
    }

    Ok(())
}

/// Recurse a directory an print its contents, at their target locations
fn recurse_dir(
    options: &ListOptions,
    target_path: impl AsRef<Path>,
    root: impl AsRef<Path>,
) -> Result<(), std::io::Error> {
    let target_path = target_path.as_ref();
    let root = root.as_ref();

    for entry in root.read_dir()? {
        let entry = entry?;
        if entry.path().is_file() {
            if entry.path().file_name().unwrap() != "namespace.fig" {
                let display_path = target_path.join(strip_namespace(root, entry.path()).unwrap());
                println!(
                    "{}{path}",
                    if options.tree { "\t" } else { "" },
                    path = display_path.display()
                );
            }
        } else {
            recurse_dir(options, target_path.join(entry.file_name()), entry.path())?;
        }
    }

    Ok(())
}
