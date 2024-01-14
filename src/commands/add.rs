use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use color_eyre::{eyre::eyre, Section};

use crate::{namespace::determine_namespace, repository::RepositoryBuilder};

#[derive(Parser, Debug)]
pub struct AddOptions {
    files: Vec<PathBuf>,
    #[clap(long)]
    mock: bool,
}

pub fn add(repo_builder: RepositoryBuilder, options: &AddOptions) -> Result<()> {
    let repository = repo_builder.open()?;

    let mut io_errors: Vec<std::io::Error> = vec![];
    let mut prefix_errors: Vec<std::path::StripPrefixError> = vec![];
    for file in &options.files {
        let file = file.canonicalize();
        if let Err(e) = file {
            io_errors.push(e);
            continue;
        }
        let file = file.unwrap();

        let namespace = determine_namespace(&repository, &file)?;

        let new_path = namespace
            .location
            .join(match file.strip_prefix(&namespace.target) {
                Ok(path) => path,
                Err(e) => {
                    prefix_errors.push(e);
                    continue;
                }
            });
        if let Some(parent) = new_path.parent() {
            let parent = namespace.target.join(parent);
            if !parent.exists() {
                if let Err(e) = crate::create_dir_all!(&parent) {
                    io_errors.push(e);
                    continue;
                }
            }
        }

        let output_path = &namespace.target.join(new_path);
        if options.mock {
            println!("{} -> {}", file.display(), output_path.display());
        } else if file.is_file() {
            match crate::copy_file!(&file, output_path) {
                Ok(_) => {}
                Err(e) => io_errors.push(e),
            }
        } else if file.is_dir() {
            crate::copy_dir!(&file, output_path);
        }
    }

    let total_errors = io_errors.len() + prefix_errors.len();
    match total_errors {
        0 => Ok(()),
        _ => {
            let mut error = eyre!(
                "Adding files: {} successful, {} failed",
                options.files.len() - total_errors,
                total_errors
            );
            for err in io_errors {
                error = error.with_error(|| err);
            }
            for err in prefix_errors {
                error = error.with_error(|| err);
            }
            Err(error)
        }
    }
}
