#![feature(fs_try_exists)]

use core::panic;
use directories::ProjectDirs;
use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};
extern crate term;
use term::*;

pub fn remove(path: PathBuf, verbose: bool) {
    let mut terminal = term::stdout().unwrap();

    let target: PathBuf = data_folder_path()
        .unwrap()
        .join(&path)
        .to_str()
        .unwrap()
        .replace("\\\\?\\", "")
        .into();
    if target.is_dir() {
        if verbose {
            print!("Removing namespace ");
            terminal.attr(Attr::Bold).unwrap();
            println!("{}", path.to_str().unwrap().replace("\\", "."));
            terminal.reset().unwrap();
        }

        // Delete the dir.
        let status = fs::remove_dir_all(&target);
        if status.is_err() {
            panic!(
                "Failed to remove namespace {}",
                path.to_str().unwrap().replace("\\", ".")
            )
        }
    } else {
        if verbose {
            print!("Removing ");
            terminal.attr(Attr::Bold).unwrap();
            println!("{}", path.display());
            terminal.reset().unwrap();
        }

        // Delete the file.
        let status = fs::remove_file(&target);
        if status.is_err() {
            panic!("Failed to remove config file {}", target.display());
        }
    }

    terminal.fg(color::GREEN).unwrap();
    println!("Done");
    terminal.reset().unwrap();

    // let mut config_borrowed = config.borrow_mut();
    // let mut origin = config_borrowed
    //     .configs
    //     .iter()
    //     .filter(|x| x.name == name && x.namespace == namespace);

    // // Make sure there is no more and no less than one config that matches the given criteria.
    // if origin.clone().count() != 1 {
    //     panic!(
    //         "There must be EXACTLY one config with the name {} in the namespace {}",
    //         name, namespace
    //     );
    // }
    // let origin = origin.next().unwrap().clone();

    // // Delete the symlink in origin.
    // if verbose {
    //     let mut terminal = term::stdout().unwrap();
    //     print!("Deleting symlink ");
    //     terminal.attr(Attr::Bold).unwrap();
    //     println!("{}", origin.origin_path.display());
    //     terminal.reset().unwrap();
    // }
    // fs::remove_file(&origin.origin_path).unwrap();

    // // Copy the file to original location.
    // print_if(
    //     verbose,
    //     &format!(
    //         "Moving file {} back to {}",
    //         origin.path.display(),
    //         origin.origin_path.display()
    //     ),
    // );
    // fs::copy(&origin.path, &origin.origin_path).unwrap();

    // // Delete the config file saved in fig data.
    // fs::remove_file(&origin.path).unwrap();

    // // Remove it from config.
    // config_borrowed.configs.retain(|x| x.path != origin.path);

    // // Check if there are any more configs in the namespace.
    // if namespace != ""
    //     && !config_borrowed
    //         .configs
    //         .iter()
    //         .any(|x| x.namespace == namespace)
    // {
    //     // Delete namespace
    //     let namespace_dir: &str = namespace
    //         .split(".")
    //         .into_iter()
    //         .next()
    //         .expect("There was no namespace.");

    //     let mut terminal = term::stdout().unwrap();
    //     print!("Deleting namespace ");
    //     terminal.attr(Attr::Bold).unwrap();
    //     println!("{}", namespace);
    //     terminal.reset().unwrap();

    //     let namespace_dir = data_folder_path().unwrap().join(namespace_dir);
    //     fs::remove_dir_all(namespace_dir).unwrap();
    // }
}

pub fn add(path: PathBuf, namespace: PathBuf, verbose: bool) {
    // print_if(verbose, &format!("Adding {}", path.display()));

    let mut terminal = term::stdout().unwrap();

    if verbose {
        print!("Adding ");
        terminal.attr(Attr::Bold).unwrap();
        println!("{}", path.display());
        terminal.reset().unwrap();
    }

    let data_folder_path = data_folder_path().unwrap();

    let origin_file = &path;
    let destination_file = &data_folder_path
        .join(&namespace)
        .join(origin_file.file_name().unwrap());

    // Check if a config already exists.
    if exists(destination_file) {
        panic!("That config already exists");
    }

    // Make sure directory exists.
    fs::create_dir_all(data_folder_path.join(&namespace)).unwrap();

    // Create hard link to target.
    fs::hard_link(origin_file, destination_file).unwrap();
    // std::process::Command::new("cmd")
    //     .arg("/C")
    //     .arg("mklink")
    //     .arg(destination_file.to_str().unwrap())
    //     .arg(origin_file.to_str().unwrap().replace("/", "\\"))
    //     .status()
    //     .expect("Failed to create link");

    // Copy original file into data_dir.
    // fs::copy(origin_file, destination_file).unwrap();

    // // Delete original file.
    // fs::remove_file(origin_file).unwrap();

    // // Create symbolic link to destination file at origin file.
    // let exit = std::process::Command::new("cmd")
    //     .arg("/C") // Command line.
    //     .arg("mklink") // Create symbolic link.
    //     //.arg("/D") // Directories only.
    //     .arg(origin_file.to_str().unwrap().replace("/", "\\"))
    //     .arg(destination_file.display().to_string())
    //     .status()
    //     .expect("Error creating symbolic link.");

    // if !exit.success() {
    //     println!("Error creating symbolic link. Aborting...");

    //     // Undo file operations.
    //     fs::copy(destination_file, origin_file).unwrap();
    //     fs::remove_file(destination_file).unwrap();

    //     panic!()
    // }

    // // print_if(
    // //     verbose,
    // //     &format!(
    // //         "Created config file in {}\nDone.",
    // //         destination_file.display(),
    // //     ),
    // // );
    // if verbose {
    //     print!("Created config file in ");
    //     terminal.attr(Attr::Bold);
    //     println!("{}", destination_file.display());
    //     terminal.reset();

    //     println!("Done.");
    // }

    // // Everything succeeded. Add the config to the FigConfig
    // config.borrow_mut().configs.push(FigConfigFile {
    //     origin_path: env::current_dir()
    //         .unwrap()
    //         .join(origin_file)
    //         .to_str() // Convert it to string so
    //         .unwrap()
    //         .replace("/", "\\") // we can do this.
    //         .into(), // convert back into PathBuf
    //     path: pathdiff::diff_paths(destination_file, data_folder_path).unwrap(),
    // });
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

pub fn list() {
    let data_folder_path = data_folder_path().unwrap();

    let entries = fs::read_dir(&data_folder_path).unwrap();

    for entry in entries {
        list_dir(entry.unwrap(), &data_folder_path);
    }
}

fn list_dir(dir: DirEntry, data_folder_path: &PathBuf) {
    if dir.path().is_file() {
        let namespace_path = pathdiff::diff_paths(dir.path(), data_folder_path).unwrap();
        let file_name = namespace_path.file_name().unwrap().to_str().unwrap();
        let mut namespace_path = namespace_path.parent().unwrap().display().to_string();
        if namespace_path != "" {
            namespace_path.push('\\');
        }
        let mut terminal = term::stdout().unwrap();
        terminal.fg(color::RED).unwrap();
        print!("{namespace_path}");
        terminal.fg(color::BRIGHT_GREEN).unwrap();
        terminal.attr(Attr::Bold).unwrap();
        println!("{file_name}");
        terminal.reset().unwrap();
    } else {
        for entry in fs::read_dir(dir.path()).unwrap() {
            list_dir(entry.unwrap(), data_folder_path);
        }
    }
}

pub fn open(path: PathBuf) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "start".to_string());

    // Run editor with the config file.
    let config_file = data_folder_path()
        .unwrap()
        .join(path)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("\\\\?\\", "");

    std::process::Command::new(editor)
        .arg(config_file)
        .status()
        .expect("Error opening config file. Try qualifying the EDITOR variable to a full path.");
}

pub fn exists(file: &PathBuf) -> bool {
    let exists = fs::try_exists(file);
    exists.unwrap()
}

pub fn dir_exists(dir: &PathBuf) -> bool {
    let exists = fs::read_dir(dir);
    exists.is_ok()
}
