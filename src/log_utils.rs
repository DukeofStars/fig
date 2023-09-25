#[macro_export]
macro_rules! create_dir_all {
    ($path:expr) => {{
        std::fs::create_dir_all($path)
            .map_err(|e| {
                tracing::error!("Failed to create directory '{dir}'", dir = $path.display());
                e
            })
            .map(|a| {
                tracing::debug!("Created directory '{path}'", path = $path.display());
                a
            })
    }};
}
#[macro_export]
macro_rules! create_dir_all_if_not_exists {
    ($path:expr) => {{
        if !$path.exists() {
            $crate::create_dir_all!($path)
        } else {
            Ok(())
        }
    }};
}

#[macro_export]
macro_rules! remove_dir_all {
    ($path:expr) => {{
        std::fs::remove_dir_all($path)
            .map_err(|e| {
                tracing::error!("Failed to remove directory '{dir}'", dir = $path.display());
                e
            })
            .map(|a| {
                tracing::debug!("Removed directory '{path}'", path = $path.display());
                a
            })
    }};
}

#[macro_export]
macro_rules! copy_file {
    ($from:expr, $to:expr) => {{
        std::fs::copy($from, $to)
            .map_err(|e| {
                tracing::error!(
                    "Failed to copy path '{from}' to '{parent}'",
                    from = $from.display(),
                    parent = $to.parent().unwrap_or_else(|| $to).display()
                );
                e
            })
            .map(|a| {
                tracing::debug!(
                    "Copied path '{from}' to '{parent}'",
                    from = $from.display(),
                    parent = $to.parent().unwrap_or_else(|| $to).display()
                );
                a
            })
    }};
}

#[macro_export]
macro_rules! copy_dir {
    ($from:expr, $to:expr) => {{
        // Files to be copied
        let mut files = vec![];
        // Iterator function
        fn iterate_dir(
            files: &mut Vec<PathBuf>,
            path: &PathBuf,
            depth_guard: u8,
        ) -> Result<(), std::io::Error> {
            for file in $crate::read_dir!(&path)? {
                if let Ok(file) = file {
                    let path = file.path();
                    if path.is_file() {
                        files.push(path);
                    } else {
                        iterate_dir(files, &path, depth_guard - 1)?;
                    }
                } else {
                    tracing::warn!("Skipping file, io error: '{err}'", err = file.unwrap_err());
                }
            }
            Ok(())
        }

        iterate_dir(&mut files, &($from), 20)?;

        let mut copied_files = vec![];

        for file in &files {
            let stripped_path = if let Ok(stripped_path) = file.strip_prefix($from) {
                stripped_path
            } else {
                tracing::warn!(
                    "Failed to strip '{}' from '{}'",
                    $from.display(),
                    file.display()
                );
                continue;
            };
            let dst = $to.join(stripped_path);

            if let Some(parent) = dst.parent() {
                let parent = $to.join(parent);
                if !parent.exists() {
                    let _ = $crate::create_dir_all!(&parent);
                }
            }

            if let Ok(_) = $crate::copy_file!(file, &dst) {
                copied_files.push(dst);
            } else {
                continue;
            }
        }

        tracing::info!(
            "Copied {} files from '{}' to '{}'",
            copied_files.len(),
            $from.display(),
            $to.display()
        );

        copied_files
    }};
}

#[macro_export]
macro_rules! read_dir {
    ($dir:expr) => {{
        $dir.read_dir().map_err(|e| {
            tracing::warn!("Failed to get contents of directory '{}'", $dir.display());
            e
        })
    }};
}
