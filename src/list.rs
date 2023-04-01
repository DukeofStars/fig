use std::path::{Path, PathBuf};

use clap::Args;
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;

use crate::{repository::Repository, strip_namespace};

#[derive(Args)]
pub struct ListOptions {
    #[clap(short, long)]
    tree: bool,
}

pub fn list(repository: &Repository, options: ListOptions) -> Result<()> {
    let namespaces = repository.namespaces()?;
    for (namespace, files) in get_all_files(repository)? {
        if options.tree {
            println!(
                "{namespace:12}: {path}",
                namespace = namespace.blue(),
                path = namespaces.get(&namespace).unwrap().display().bright_blue()
            );
        }
        for file in files {
            println!(
                "{}{path}",
                if options.tree {
                    " ".repeat(12)
                } else {
                    "".to_string()
                },
                path = file.display().green()
            );
        }
    }

    Ok(())
}

pub fn get_all_files(repository: &Repository) -> Result<Vec<(String, Vec<PathBuf>)>> {
    let mut files = vec![];
    for (name, path) in repository.namespaces()? {
        let dir = repository.dir.join(&name);
        let mut ns_files = vec![];
        recurse_dir(&mut ns_files, &path, &dir).into_diagnostic()?;
        files.push((name, ns_files));
    }

    Ok(files)
}

/// Recurse a directory an print its contents, at their target locations
fn recurse_dir(
    files: &mut Vec<PathBuf>,
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
                files.push(display_path);
            }
        } else {
            recurse_dir(files, target_path.join(entry.file_name()), entry.path())?;
        }
    }

    Ok(())
}
