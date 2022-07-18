#![feature(fs_try_exists)]
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

use clap::{Args, Parser, Subcommand};

use fig::{config_folder_path, FigConfig};

extern crate pathdiff;

fn main() {
    let cli = Cli::parse();

    let config_path = config_folder_path().unwrap().join("config.toml");

    let config_file = fs::read_to_string(&config_path);

    let config: Rc<RefCell<FigConfig>> = Rc::new(RefCell::new(match config_file {
        Ok(_) => toml::from_str(&config_file.unwrap()).unwrap(),
        Err(_) => FigConfig { configs: vec![] },
    }));

    use SubCommand::*;
    match cli.cmd {
        Add(mut add) => {
            if add.namespace.clone().unwrap_or_default() == "" {
                let path = add
                    .path
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    // Get rid of weird prefix that is put here. I don't know why it is there.
                    .replace("\\\\?\\", "");
                let base_dirs = directories::BaseDirs::new().unwrap();
                let home = base_dirs.home_dir(); //"C:/Users/Fox";
                let rel_path = pathdiff::diff_paths(&path, home).unwrap();
                let namespace = rel_path.parent();
                let namespace = if namespace.is_some() {
                    namespace.unwrap().to_str().unwrap().replace("\\", ".")
                } else {
                    "".to_string()
                };

                if namespace.contains("AppData") {
                    // redo it but this time from AppData directory
                    let app_data = home.join("AppData").join(if namespace.contains("Roaming") {
                        "Roaming"
                    } else {
                        "Local"
                    });
                    let rel_path = pathdiff::diff_paths(&path, app_data).unwrap();
                    let namespace = rel_path.parent();
                    let namespace = if namespace.is_some() {
                        namespace.unwrap().to_str().unwrap().replace("\\", ".")
                    } else {
                        "".to_string()
                    };

                    add.namespace.replace(namespace);
                } else {
                    add.namespace.replace(namespace);
                }
            }

            fig::add(
                config.clone(),
                add.path,
                add.namespace.unwrap_or_default().replace(".", "/").into(),
                add.verbose,
            );
        }
        Forget(mut forget) => {
            if forget.namespace.clone().unwrap_or_default() == "" {
                let project_dirs =
                    directories::ProjectDirs::from("me", "dukeofstars", "fig").unwrap();
                let path = project_dirs
                    .data_dir()
                    .join(forget.name.clone().unwrap())
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    // Get rid of weird prefix that is put here. I don't know why it is there.
                    .replace("\\\\?\\", "");
                let base_dirs = directories::BaseDirs::new().unwrap();
                let home = base_dirs.home_dir(); //"C:/Users/{User Name}";
                let rel_path = pathdiff::diff_paths(&path, home).unwrap();
                let namespace = rel_path.parent();
                let namespace = if namespace.is_some() {
                    namespace.unwrap().to_str().unwrap().replace("\\", ".")
                } else {
                    "".to_string()
                };

                // This is added when we create the 'path' variable, so it must be removed.
                let mut namespace = namespace.replace("dukeofstars.fig.data", "");

                if namespace.contains("AppData") {
                    namespace = namespace.replace("AppData.", "");
                    if namespace.contains("Roaming") {
                        namespace = namespace.replace("Roaming.", "");
                    }
                }

                namespace = namespace.trim_start_matches('.').to_string();

                forget.namespace.replace(namespace);
            }

            forget.name = Some(
                PathBuf::from(forget.name.unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );

            fig::remove(
                config.clone(),
                forget.name,
                forget.namespace.unwrap_or_default(),
                !forget.quiet, // Invert quiet so the fig::remove can treat it as a 'verbose' flag.
            );
        }
        List => {
            fig::list(config.clone());
        }
        Open(mut open) => {
            if open.namespace.is_some() {
                let namespace = PathBuf::from(open.namespace.unwrap());
                open.path = namespace.join(open.path);
            }
            fig::open(open.path);
        }
    }

    let config: FigConfig = config.borrow().clone();

    fs::write(
        config_path,
        toml::to_string_pretty::<FigConfig>(&config).unwrap(),
    )
    .unwrap();
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Add(AddArgs),
    Forget(ForgetArgs),
    List,
    Open(OpenArgs),
}

#[derive(Args, Debug)]
struct AddArgs {
    #[clap(short, long)]
    verbose: bool,
    #[clap(parse(from_os_str))]
    path: PathBuf,
    #[clap(short, long)]
    namespace: Option<String>,
}

#[derive(Args, Debug)]
struct ForgetArgs {
    #[clap(short, long)]
    quiet: bool,
    #[clap(name = "name", parse(from_str))]
    name: Option<String>,
    #[clap(short, long)]
    namespace: Option<String>,
}

#[derive(Args, Debug)]
struct OpenArgs {
    #[clap(name = "name", parse(from_os_str))]
    path: PathBuf,
    #[clap(short, long)]
    namespace: Option<String>,
}
