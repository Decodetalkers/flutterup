use std::{env, path::PathBuf};

use constenv::{PathMessage, ProcessResult};
mod clonetoolchain;
mod config;
mod constenv;
mod wrapper;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn flutterupinfo() {
    let matches = clap::Command::new("flutterup")
        .about("Flutter wrapper")
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Haruhi Suzumiya")
        .subcommand(
            clap::Command::new("upgrade")
                .long_flag("upgrade")
                .about("Upgrade flutter"),
        )
        .subcommand(
            clap::Command::new("install")
                .long_flag("install")
                .about("Install flutter"),
        )
        .subcommand(
            clap::Command::new("showconfig")
                .long_flag("showconfig")
                .about("Show the config of flutterup"),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("upgrade", _)) => {
            if let Ok(ProcessResult::Successded) =
                wrapper::run_wrapper("flutter", &["upgrade".to_string()])
            {
            } else {
                eprintln!("SomeError");
            }
        }
        Some(("install", _)) => {
            let PathMessage::GetPath { path } = &*constenv::CLONEPATH else {
                unreachable!()
            };
            let flutterpath = PathBuf::from(path).join("bin").join("flutter");
            if flutterpath.exists() {
                println!("Seems flutter is already installed");
                return;
            }
            match clonetoolchain::flutter_clone() {
                Ok(messages) => {
                    if let ProcessResult::Successded = messages {
                        println!("Flutter has been installed");
                    } else {
                        eprintln!("Some Error");
                    }
                }
                Err(e) => {
                    eprintln!("SomeError : {e}");
                }
            }
        }
        Some(("showconfig", _)) => {
            let config = config::CONFIG.clone();
            let (branch, dir) = {
                config
                    .map(|conf| {
                        (
                            conf.branch.unwrap_or("stable".to_string()),
                            conf.flutter_sdk_dir
                                .unwrap_or("~/.local/share/flutterup".to_string()),
                        )
                    })
                    .unwrap_or(("stable".to_string(), "~/.local/share/flutterup".to_string()))
            };
            println!("flutter sdk branch is {branch}");
            println!("cloned dir is in {dir}");
        }
        _ => unimplemented!(),
    }
}

fn main() {
    if let PathMessage::SomeError { error } = &*constenv::CLONEPATH {
        eprintln!("SomeError with your environment: {error}");
    }
    let PathMessage::GetPath { path } = &*constenv::CLONEPATH else {
        unreachable!()
    };
    let pathenv = std::env::var("PATH").unwrap();
    let newpath = format!("{path}/bin:{pathenv}");
    unsafe { std::env::set_var("PATH", newpath) };
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    let first = args[0].split('/').next_back().unwrap();

    match first {
        "flutterup" => {
            flutterupinfo();
        }
        "flutter" => {
            let options = &args[1..];
            let flutterbin = format!("{path}/bin/flutter");
            match wrapper::run_wrapper(&flutterbin, options) {
                Ok(messages) => match messages {
                    constenv::ProcessResult::Others => {
                        eprintln!("Maybe stopped?");
                    }
                    constenv::ProcessResult::NotFound => {
                        let PathMessage::GetPath { path } = &*constenv::CLONEPATH else {
                            unreachable!()
                        };
                        let flutterpath = PathBuf::from(path).join("bin").join("flutter");
                        if flutterpath.exists() {
                            wrapper::run_wrapper(flutterpath.to_str().unwrap(), options).unwrap();
                        }
                        match clonetoolchain::flutter_clone() {
                            Ok(messages) => {
                                if let ProcessResult::Successded = messages {
                                    println!("Flutter has been installed");
                                } else {
                                    eprintln!("Some Error");
                                }
                            }
                            Err(e) => {
                                eprintln!("SomeError : {e}");
                            }
                        }
                    }
                    constenv::ProcessResult::Successded => {}
                },
                Err(e) => {
                    eprintln!("SomethingError :{e}");
                }
            };
        }
        "dart" => {
            let options = &args[1..];
            let dartbin = format!("{path}/bin/dart");
            match wrapper::run_wrapper(&dartbin, options) {
                Ok(messages) => match messages {
                    constenv::ProcessResult::Others => {
                        eprintln!("Maybe stopped?");
                    }
                    constenv::ProcessResult::NotFound => {
                        let PathMessage::GetPath { path } = &*constenv::CLONEPATH else {
                            unreachable!()
                        };
                        let flutterpath = PathBuf::from(path).join("bin").join("dart");
                        if flutterpath.exists() {
                            wrapper::run_wrapper(flutterpath.to_str().unwrap(), options).unwrap();
                        }
                        match clonetoolchain::flutter_clone() {
                            Ok(messages) => {
                                if let ProcessResult::Successded = messages {
                                    println!("Flutter has been installed");
                                } else {
                                    eprintln!("Some Error");
                                }
                            }
                            Err(e) => {
                                eprintln!("SomeError : {e}");
                            }
                        }
                    }
                    constenv::ProcessResult::Successded => {}
                },
                Err(e) => {
                    eprintln!("SomethingError :{e}");
                }
            };
        }
        _ => unimplemented!(),
    }
}
