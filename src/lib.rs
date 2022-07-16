use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FigConfig {
    pub configs: Vec<FigConfigFile>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FigConfigFile {
    pub name: String,
    pub origin_path: PathBuf,
    pub path: PathBuf,
    pub namespace: String,
}

pub fn remove(config: &mut FigConfig, name: Option<String>, namespace: String, verbose: bool) {
    match name {
        Some(name) => {
            print_if(verbose, &format!("Removing {}", name));

            let mut origin = config
                .configs
                .iter()
                .filter(|x| x.name == name && x.namespace == namespace);
            if origin.clone().count() != 1 {
                panic!(
                    "There must be EXACTLY one config with the name {} in the namespace {}",
                    name, namespace
                );
            }
            let origin = origin.next().unwrap().clone();

            // Delete the config file saved in fig data.
            fs::remove_file(&origin.path).unwrap();

            // Remove it from config.
            config.configs.retain(|x| x.path != origin.path);
        }
        None => {
            if namespace == "" {
                panic!("You must specify a namespace or a name when forgetting a config.");
            }
            print_if(
                verbose,
                &format!("Removing all configs in namespace {}", namespace),
            );

            // Convert namespace to path.
            let namespace_p: PathBuf = namespace.replace(".", "/").into();

            // Delete the config files saved in fig data.
            fs::remove_dir_all(data_folder_path().unwrap().join(&namespace_p)).expect(&format!(
                "Failed to delete directory {}",
                data_folder_path().unwrap().join(&namespace_p).display(),
            ));

            // Remove it from config.
            config
                .configs
                .retain(|x| !x.namespace.starts_with(&namespace));
        }
    }
}

pub fn add(config: &mut FigConfig, path: PathBuf, namespace: PathBuf, verbose: bool) {
    print_if(verbose, &format!("Adding {}", path.display()));

    let config_folder_path = data_folder_path().unwrap();

    let origin_file = &path;
    let destination_file = &config_folder_path
        .join(&namespace)
        .join(origin_file.file_name().unwrap());

    // Check if a config already exists.
    if config.configs.iter().any(|x| x.path == *destination_file) {
        panic!("That config file already exists!");
    }

    // Make sure directory exists.
    fs::create_dir_all(config_folder_path.join(&namespace)).unwrap();

    // Create symbolic link to file in original folder using mklink tool.
    let _ = std::process::Command::new("cmd")
        .arg("/C") // Command line.
        .arg("mklink") // Create symbolic link.
        //.arg("/D") // Directories only.
        .arg(destination_file.display().to_string())
        .arg(origin_file.display().to_string())
        .status()
        .expect("Error creating symbolic link.");

    print_if(
        verbose,
        &format!(
            "Created config file in {}\nDone.",
            &destination_file.display(),
        ),
    );

    // Everything succeeded. Add the config to the FigConfig
    config.configs.push(FigConfigFile {
        name: origin_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        origin_path: origin_file.to_path_buf().canonicalize().unwrap(),
        path: destination_file.to_path_buf(),
        namespace: namespace.to_str().unwrap_or_default().replace("/", "."),
    });
}

pub fn data_folder_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("me", "dukeofstars", "fig") {
        let dir = proj_dirs.data_dir().to_path_buf();
        let result = dir.canonicalize();
        if result.is_err() {
            fs::create_dir_all(&dir).expect("Error creating config folder.");
            let result = dir.canonicalize();
            Some(result.unwrap())
        } else {
            Some(result.unwrap())
        }
    } else {
        None
    }
}

pub fn config_folder_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("me", "dukeofstars", "fig") {
        let dir = proj_dirs.config_dir().to_path_buf();
        let result = dir.canonicalize();
        if result.is_err() {
            fs::create_dir_all(&dir).expect("Error creating config folder.");
            let result = dir.canonicalize();
            Some(result.unwrap())
        } else {
            Some(result.unwrap())
        }
    } else {
        None
    }
}

fn print_if(verbose: bool, msg: &str) {
    if verbose {
        println!("{}", msg);
    }
}
