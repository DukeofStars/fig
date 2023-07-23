#[macro_export]
macro_rules! create_dir_all {
    ($path:expr) => {{
        use log::{error, trace};
        std::fs::create_dir_all($path)
            .map_err(|e| {
                error!("Failed to create directory '{dir}'", dir = $path.display());
                e
            })
            .map(|a| {
                trace!("Created directory '{path}'", path = $path.display());
                a
            })
    }};
}

#[macro_export]
macro_rules! copy_file {
    ($from:expr, $to:expr) => {{
        use log::{error, trace};
        std::fs::copy($from, $to)
            .map_err(|e| {
                error!(
                    "Failed to copy path '{from}' to '{parent}'",
                    from = $from.display(),
                    parent = $to.parent().unwrap_or_else(|| $to).display()
                );
                e
            })
            .map(|a| {
                trace!(
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
        fn iterate_dir(files: &mut Vec<PathBuf>, path: &PathBuf, depth_guard: u8) {
            for file in ::fig::read_dir!(&path).expect("Failed to read directory") {
                if let Ok(file) = file {
                    let path = file.path();
                    if path.is_file() {
                        files.push(path);
                    } else {
                        iterate_dir(files, &path, depth_guard - 1);
                    }
                } else {
                    log::warn!("Skipping file, io error: '{err}'", err = file.unwrap_err());
                }
            }
        }

        iterate_dir(&mut files, &($from), 20);

        let mut copied_files = vec![];

        for file in &files {
            let stripped_path = if let Ok(stripped_path) = file.strip_prefix($from) {
                stripped_path
            } else {
                log::warn!(
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
                    let _ = ::fig::create_dir_all!(&parent);
                }
            }

            if let Ok(_) = ::fig::copy_file!(file, &dst) {
                copied_files.push(dst);
            } else {
                continue;
            }
        }

        log::trace!(
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
            log::warn!("Failed to get contents of directory '{}'", $dir.display());
            e
        })
    }};
}
