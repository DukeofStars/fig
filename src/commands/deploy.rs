use std::path::{Path, PathBuf};

use clap::Args;
use color_eyre::{eyre::Context, Result};

use fig::repository::RepositoryBuilder;

/// Deploy files from the configuration repository to your system.
#[derive(Debug, Args)]
pub struct DeployOptions {
    #[clap(short, long)]
    verbose: bool,
}

pub fn deploy(repo_builder: RepositoryBuilder, options: &DeployOptions) -> Result<()> {
    let repository = repo_builder.open()?;

    let namespaces = repository.namespaces()?;

    for namespace in &namespaces {
        let mut files = vec![];
        get_files(&namespace.location, &namespace.location, &mut files, 10)?;
        for file in files {
            let dest = namespace.target.join(&file);
            let src = namespace.location.join(&file);

            // Make sure dest directory exists
            if let Some(parent) = dest.parent() {
                if options.verbose {
                    println!("Creating directory: {parent}", parent = parent.display());
                }
                crate::create_dir_all!(parent)?;
            }

            if options.verbose {
                println!("Copying: '{}' to '{}'", src.display(), dest.display())
            }
            crate::copy_file!(&src, &dest).context(format!(
                "Failed to copy '{}' to '{}'",
                src.display(),
                dest.display()
            ))?;
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
    let path = entry
        .strip_prefix(parent_dir)
        .context("Failed to strip path prefix")?;
    files.push(path.to_path_buf());
    Ok(())
}
