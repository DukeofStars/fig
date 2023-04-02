use std::fs;

use miette::Result;

use fig::{repository::Repository, Error::*};

pub fn purge(repository: &Repository) -> Result<()> {
    fs::remove_dir_all(&repository.dir).map_err(IoError)?;

    Ok(())
}
