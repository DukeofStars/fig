#![feature(fs_try_exists)]
use std::{path::PathBuf, process};

use clap::{Args, Parser, Subcommand};

extern crate pathdiff;

fn main() {
    let cli = Cli::parse();

    use SubCommand::*;

    if cli.path {
        let path = fig::config_folder_path().expect("Failed to obtain config folder");
        println!("{}", path.display());
    }

    if let Some(cmd) = cli.cmd {
        match cmd {
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
                        let app_data =
                            home.join("AppData").join(if namespace.contains("Roaming") {
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
                    add.path,
                    add.namespace.unwrap_or_default().replace(".", "/").into(),
                    add.verbose,
                );
            }
            Forget(forget) => {
                let path = PathBuf::from(forget.namespace.unwrap_or_default().replace(".", "\\"))
                    .join(forget.path.unwrap_or_default());

                fig::remove(
                    path,
                    !forget.quiet, // Invert quiet so the fig::remove can treat it as a 'verbose' flag.
                );
            }
            List => {
                fig::list();
            }
            Open(mut open) => {
                if open.namespace.is_some() {
                    let namespace = PathBuf::from(open.namespace.unwrap());
                    open.path = namespace.join(open.path);
                }
                fig::open(open.path);
            }
            // Change directory
            Cd => {
                let path = fig::config_folder_path()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("\\\\?\\", "");
                #[cfg(windows)]
                let status = process::Command::new("cmd")
                    .current_dir(path)
                    .status()
                    .unwrap();
                #[cfg(not(windows))]
                let status = process::Command::new("sh")
                    .current_dir(path)
                    .status()
                    .unwrap();
                process::exit(status.code().unwrap());
            }
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Option<SubCommand>,
    #[clap(short, long)]
    path: bool,
}

#[derive(Subcommand)]
enum SubCommand {
    Add(AddArgs),
    Forget(ForgetArgs),
    List,
    Open(OpenArgs),
    Cd,
}

#[derive(Args, Debug)]
struct AddArgs {
    #[clap(short, long)]
    verbose: bool,
    path: PathBuf,
    #[clap(short, long)]
    namespace: Option<String>,
}

#[derive(Args, Debug)]
struct ForgetArgs {
    #[clap(short, long)]
    quiet: bool,
    #[clap(name = "name")]
    path: Option<String>,
    #[clap(short, long)]
    namespace: Option<String>,
}

#[derive(Args, Debug)]
struct OpenArgs {
    #[clap(name = "name")]
    path: PathBuf,
    #[clap(short, long)]
    namespace: Option<String>,
}
