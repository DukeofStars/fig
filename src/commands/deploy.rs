use std::{fs, path::PathBuf};

use clap::Args;
use miette::Result;

use fig::{repository::Repository, Error::*};

#[derive(Args)]
pub struct DeployOptions {
    #[clap(short, long)]
    verbose: bool,
}

pub fn deploy(repository: &Repository, options: DeployOptions) -> Result<()> {
    let namespaces = repository.namespaces()?;
    let dir = repository.dir.to_path_buf();

    for (name, namespace_path) in &namespaces {
        let namespace_dir = dir.join(name);
        let mut files = vec![];
        get_files(&dir, &namespace_dir, &mut files, 10)?;
        for file in files {
            let file = file
                .to_str()
                .unwrap()
                .trim_start_matches("\\")
                .trim_start_matches("/");

            let path = file
                .strip_prefix(name)
                .ok_or_else(|| {
                    println!("ERROR_LOG, try running with --verbose for more info");
                    dbg!(&namespace_dir, &namespace_path, &file);
                })
                .expect("Failed to strip prefix")
                .trim_start_matches("/");

            let dest = namespace_path.join(&path);
            let src = namespace_dir.join(&path);

            // Make sure dest directory exists
            if let Some(parent) = dest.parent() {
                if options.verbose {
                    println!("Creating directory: {parent}", parent = parent.display());
                }
                fs::create_dir_all(&parent).map_err(IoError)?;
            }

            if options.verbose {
                println!("{} -> {}", src.display(), dest.display())
            }
            fs::copy(&src, &dest).map_err(IoError).map_err(|e| {
                println!("ERROR_LOG, try running with --verbose for more info");
                dbg!(&namespace_dir, &namespace_path, &path);
                e
            })?;
        }
    }

    Ok(())
}

fn get_files(
    parent_dir: &PathBuf,
    entry: &PathBuf,
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
        for entry in entry.read_dir().map_err(IoError)? {
            let entry = entry.map_err(IoError)?.path();
            get_files(parent_dir, &entry, files, depth - 1)?;
        }

        return Ok(());
    }
    // Strip fig dir
    let path = PathBuf::from(
        entry
            .to_str()
            .ok_or_else(|| PathConversionFail)?
            .trim_start_matches(parent_dir.to_str().ok_or_else(|| PathConversionFail)?),
    );
    files.push(path);
    Ok(())
}
