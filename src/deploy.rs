use std::{
    fs, io,
    path::{self, PathBuf},
};

use clap::Args;
use miette::{Diagnostic, Result};
use thiserror::Error;

use crate::{repository::Repository, Error::*};
use Error::*;

#[derive(Args)]
pub struct DeployOptions {
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error("Namespace doesn't exist")]
    #[diagnostic(code(fig::deploy::namespace_doesnt_exist_io))]
    NamespaceDoesntExistIO(#[source] io::Error),
    #[error("Namespace doesn't exist")]
    #[diagnostic(code(fig::deploy::namespace_doesnt_exist))]
    NamespaceDoesntExist,
    #[error(transparent)]
    #[diagnostic(code(fig::deploy::strip_prefix_error))]
    StripPrefixError(#[from] path::StripPrefixError),
}

pub fn deploy(repository: &Repository, options: DeployOptions) -> Result<()> {
    let namespaces = repository.namespaces()?;
    let dir = repository.dir.to_path_buf();

    for (name, namespace_path) in &namespaces {
        let namespace_dir = dir.join(name);
        let mut files = vec![];
        get_files(&dir, &namespace_dir, &mut files, 10)?;
        for file in files {
            let file = file.strip_prefix("\\").map_err(StripPrefixError)?;

            let path = file.strip_prefix(name).map_err(StripPrefixError)?;
            let dest = namespace_path.join(&path);
            let src = namespace_dir.join(&path);
            if options.verbose {
                println!("{} -> {}", src.display(), dest.display())
            }
            fs::copy(&src, &dest).map_err(IoError)?;
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
