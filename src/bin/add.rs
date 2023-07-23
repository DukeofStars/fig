use std::path::PathBuf;

use clap::Parser;
use glob::glob;
use log::error;
use miette::Diagnostic;
use miette::IntoDiagnostic;
use miette::Result;
use thiserror::Error;

use fig::{determine_namespace, repository::Repository};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("An error occurred iterating glob paths")]
    #[diagnostic(code(fig::add::glob_error))]
    GlobError(#[from] glob::GlobError),
    #[error("An error occurred parsing glob strings")]
    #[diagnostic(code(fig::add::pattern_error))]
    PatternError(#[from] glob::PatternError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("The file '{}' does not exist", .0.display())]
    #[diagnostic(code(fig::add::file_doesnt_exist))]
    FileDoesntExist(PathBuf),
    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error("Adding files failed with {} errors", .0.len())]
    Errors(#[related] Vec<Self>),
}

#[derive(Parser, Debug)]
pub struct Options {
    files: Vec<String>,
    #[clap(long)]
    mock: bool,
}

fn main() -> Result<()> {
    let repository = Repository::open()?;
    let options = Options::parse();

    let mut files: Vec<PathBuf> = vec![];
    let mut warnings: Vec<Error> = vec![];
    for file in options.files {
        for entry in glob(file.as_str()).into_diagnostic()? {
            // if let Ok(path) = entry {
            //     files.push(path);
            // } else if let  {
            //     warnings.push(Error::from(entry))
            // }
            match entry {
                Ok(path) => files.push(path),
                Err(e) => warnings.push(e.into()),
            }
        }
    }

    for file in files {
        let file = file.canonicalize();
        if let Err(e) = file {
            warnings.push(e.into());
            continue;
        }
        let file = file.unwrap();

        if !file.exists() {
            warnings.push(Error::FileDoesntExist(file));
            continue;
        }
        let namespace = determine_namespace(&repository, &file)?;

        let new_path = namespace
            .location
            .join(match file.strip_prefix(&namespace.target) {
                Ok(path) => path,
                Err(e) => {
                    warnings.push(e.into());
                    continue;
                }
            });
        if let Some(parent) = new_path.parent() {
            let parent = namespace.target.join(parent);
            if !parent.exists() {
                if let Err(e) = fig::create_dir_all!(&parent) {
                    warnings.push(e.into());
                    continue;
                }
            }
        }

        let output_path = &namespace.target.join(new_path);
        if options.mock {
            println!("{} -> {}", file.display(), output_path.display())
        } else if file.is_file() {
            match fig::copy_file!(&file, &output_path) {
                Ok(_) => {}
                Err(e) => warnings.push(e.into()),
            }
        } else if file.is_dir() {
            fig::copy_dir!(&file, &output_path);
        }
    }

    match warnings.len() {
        0 => Ok(()),
        1 => Err(warnings.remove(0))?,
        _ => Err(Error::Errors(warnings))?,
    }
}
