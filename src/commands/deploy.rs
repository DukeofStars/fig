use std::path::{Path, PathBuf};

use clap::Args;
use color_eyre::eyre::eyre;
use color_eyre::Result;

use crate::repository::Repository;

/// Deploy files from the configuration repository to your system.
#[derive(Debug, Args)]
pub struct DeployOptions {
    #[clap(short, long)]
    verbose: bool,
}

pub fn deploy(repository: &Repository, options: &DeployOptions) -> Result<()> {
    let namespaces = repository.namespaces()?;

    for namespace in &namespaces {
        let mut files = vec![];
        get_files(&namespace.location, &namespace.location, &mut files, 10)?;
        for file in files {
            let file = file
                .to_str()
                .unwrap()
                .trim_start_matches('\\')
                .trim_start_matches('/');

            let path = file;

            let dest = namespace.target.join(path);
            let src = namespace.location.join(path);

            // Make sure dest directory exists
            if let Some(parent) = dest.parent() {
                if options.verbose {
                    println!("Creating directory: {parent}", parent = parent.display());
                }
                crate::create_dir_all!(parent)?;
            }

            if options.verbose {
                println!("Copying '{}' to '{}'", src.display(), dest.display())
            }
            crate::copy_file!(&src, &dest)?;
        }
    }

    Ok(())
}

fn get_files(
    parent_dir: &PathBuf,
    entry: &Path,
    files: &mut Vec<PathBuf>,
    depth: u8,
) -> Result<()> {
    if depth == 0 {
        return Ok(());
    } else if let Some(extension) = entry.extension() {
        if extension == "fig" {
            return Ok(());
        }
    } else if entry.is_dir() {
        for entry in entry.read_dir()? {
            let entry = entry?.path();
            get_files(parent_dir, &entry, files, depth - 1)?;
        }

        return Ok(());
    }
    // Strip fig dir
    let path = PathBuf::from(
        entry
            .to_str()
            .ok_or_else(|| eyre!("Failed to convert path"))?
            .trim_start_matches(
                parent_dir
                    .to_str()
                    .ok_or_else(|| eyre!("Failed to convert path"))?,
            ),
    );
    files.push(path);
    Ok(())
}
