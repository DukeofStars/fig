use std::{cell::RefCell, env, fs, path::PathBuf, rc::Rc};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

extern crate term;
use term::*;

type Config = Rc<RefCell<FigConfig>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
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

pub fn remove(config: Config, name: Option<String>, namespace: String, verbose: bool) {
    match name {
        Some(name) => {
            if verbose {
                let mut terminal = term::stdout().unwrap();
                print!("Removing ");
                terminal.attr(Attr::Bold).unwrap();
                println!("{}", name);
                terminal.reset().unwrap();
            }

            let mut config_borrowed = config.borrow_mut();
            let mut origin = config_borrowed
                .configs
                .iter()
                .filter(|x| x.name == name && x.namespace == namespace);

            // Make sure there is no more and no less than one config that matches the given criteria.
            if origin.clone().count() != 1 {
                panic!(
                    "There must be EXACTLY one config with the name {} in the namespace {}",
                    name, namespace
                );
            }
            let origin = origin.next().unwrap().clone();

            // Delete the symlink in origin.
            if verbose {
                let mut terminal = term::stdout().unwrap();
                print!("Deleting symlink ");
                terminal.attr(Attr::Bold).unwrap();
                println!("{}", origin.origin_path.display());
                terminal.reset().unwrap();
            }
            fs::remove_file(&origin.origin_path).unwrap();

            // Copy the file to original location.
            print_if(
                verbose,
                &format!(
                    "Moving file {} back to {}",
                    origin.path.display(),
                    origin.origin_path.display()
                ),
            );
            fs::copy(&origin.path, &origin.origin_path).unwrap();

            // Delete the config file saved in fig data.
            fs::remove_file(&origin.path).unwrap();

            // Remove it from config.
            config_borrowed.configs.retain(|x| x.path != origin.path);

            // Check if there are any more configs in the namespace.
            if namespace != ""
                && !config_borrowed
                    .configs
                    .iter()
                    .any(|x| x.namespace == namespace)
            {
                // Delete namespace
                let namespace_dir: &str = namespace
                    .split(".")
                    .into_iter()
                    .next()
                    .expect("There was no namespace.");

                let mut terminal = term::stdout().unwrap();
                print!("Deleting namespace ");
                terminal.attr(Attr::Bold).unwrap();
                println!("{}", namespace);
                terminal.reset().unwrap();

                let namespace_dir = data_folder_path().unwrap().join(namespace_dir);
                fs::remove_dir_all(namespace_dir).unwrap();
            }
        }
        None => {
            if namespace == "" {
                panic!("You must specify a namespace or a name when forgetting a config.");
            }
            if verbose {
                let mut terminal = term::stdout().unwrap();
                print!("Removing all configs in namespace ");
                terminal.attr(Attr::Bold).unwrap();
                println!("{}", namespace);
                terminal.reset().unwrap();
            }

            // Iter over all configs in the namespace and delete them all.
            config
                .borrow()
                .configs
                .iter()
                .filter(|x| x.namespace == namespace)
                .for_each(|x| {
                    remove(
                        config.clone(),
                        Some(x.name.clone()),
                        namespace.clone(),
                        verbose,
                    );
                });

            config
                .borrow_mut()
                .configs
                .retain(|x| x.namespace != namespace);
        }
    }
}

pub fn add(config: Config, path: PathBuf, namespace: PathBuf, verbose: bool) {
    print_if(verbose, &format!("Adding {}", path.display()));

    let data_folder_path = data_folder_path().unwrap();

    let origin_file = &path;
    let destination_file = &data_folder_path
        .join(&namespace)
        .join(origin_file.file_name().unwrap());

    // Check if a config already exists.
    if config
        .borrow()
        .configs
        .iter()
        .any(|x| x.path == *destination_file)
    {
        panic!("That config file already exists!");
    }

    // Make sure directory exists.
    fs::create_dir_all(data_folder_path.join(&namespace)).unwrap();

    // Copy original file into data_dir.
    fs::copy(origin_file, destination_file).unwrap();

    // Delete original file.
    fs::remove_file(origin_file).unwrap();

    // Create symbolic link to destination file at origin file.
    let exit = std::process::Command::new("cmd")
        .arg("/C") // Command line.
        .arg("mklink") // Create symbolic link.
        //.arg("/D") // Directories only.
        .arg(origin_file.to_str().unwrap().replace("/", "\\"))
        .arg(destination_file.display().to_string())
        .status()
        .expect("Error creating symbolic link.");

    if !exit.success() {
        println!("Error creating symbolic link. Aborting...");

        // Undo file operations.
        fs::copy(destination_file, origin_file).unwrap();
        fs::remove_file(destination_file).unwrap();

        panic!()
    }

    print_if(
        verbose,
        &format!(
            "Created config file in {}\nDone.",
            destination_file.display(),
        ),
    );

    // Everything succeeded. Add the config to the FigConfig
    config.borrow_mut().configs.push(FigConfigFile {
        name: origin_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace("/", "\\"),
        origin_path: env::current_dir()
            .unwrap()
            .join(origin_file)
            .to_str() // Convert it to string so
            .unwrap()
            .replace("/", "\\") // we can do this.
            .into(), // convert back into PathBuf
        path: destination_file.clone().canonicalize().unwrap(),
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

pub fn list(config: Config) {
    let mut terminal = term::stdout().unwrap();

    print!("Found ");
    terminal.fg(color::BRIGHT_CYAN).unwrap();
    terminal.attr(Attr::Bold).unwrap();
    print!("{}", config.borrow().configs.len());
    terminal.reset().unwrap();
    println!(" configs.");
    config.borrow_mut().configs.sort_by(|a, b| {
        let a: PathBuf =
            <String as Into<PathBuf>>::into(a.namespace.replace(".", "\\")).join(&a.name);
        let b: PathBuf =
            <String as Into<PathBuf>>::into(b.namespace.replace(".", "\\")).join(&b.name);
        a.cmp(&b)
    });
    config.borrow().configs.iter().for_each(|x| {
        if x.namespace != "" {
            terminal.fg(color::RED).unwrap();
            print!("{}\\", x.namespace.replace(".", "\\"));
        }
        terminal.fg(color::CYAN).unwrap();
        terminal.attr(Attr::Bold).unwrap();
        print!("{}", x.name);
        terminal.reset().unwrap();
        println!(": {}", x.origin_path.display(),)
    });
}

pub fn open(config: Config, name: Option<String>, namespace: String) {
    if name.is_none() {
        println!("Error, you must specify a name.");
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "start".to_string());

    // Run editor with the config file.
    let config_file = config
        .borrow()
        .configs
        .iter()
        .find(|x| {
            if namespace != "" {
                x.namespace == namespace && x.name == *name.as_ref().unwrap()
            } else {
                x.name == *name.as_ref().unwrap()
            }
        })
        .unwrap()
        .path
        .clone();
    std::process::Command::new(editor)
        .arg(config_file)
        .status()
        .expect("Error opening config file. Try qualifying the EDITOR variable to a full path.");
}
