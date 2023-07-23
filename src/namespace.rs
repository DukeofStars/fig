use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
}

pub struct Namespace {
    /// The output location, where files are deployed to.
    pub target: PathBuf,
    /// The physical location of the namespace, where files are stored.
    pub location: PathBuf,
}

impl Namespace {
    pub fn files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = vec![];
        self.recurse_dir(&self.location, &mut files, 50)?;
        Ok(files)
    }

    fn recurse_dir(&self, dir: &PathBuf, files: &mut Vec<PathBuf>, depth: u8) -> Result<(), Error> {
        if depth == 0 {
            panic!("Overflowed depth")
        }
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if path.file_name().unwrap() != "namespace.fig" {
                    let relative_path = path.strip_prefix(&self.location)?;
                    let display_path = self.target.join(relative_path);
                    files.push(display_path);
                }
            } else {
                self.recurse_dir(&path, files, depth - 1)?;
            }
        }

        Ok(())
    }
}
