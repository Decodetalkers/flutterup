use std::{env, path::PathBuf};

use constenv::{PathMessage, ProcessResult};
mod clonetoolchain;
mod constenv;
mod wrapper;
//use clap::Command;

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
                .about("Get upgrade"),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("upgrade", _)) => {
            if let Ok(ProcessResult::Successed) =
                wrapper::run_wrapper("flutter", &["upgrade".to_string()])
            {
            } else {
                eprintln!("SomeError");
            }
        }
        _ => unimplemented!(),
    }
}

fn main() {
    if let PathMessage::SomeError { error } = &*constenv::CLONEPATH {
        eprintln!("SomeError with your environment: {}", error);
    }
    let PathMessage::GetPath { path } = &*constenv::CLONEPATH else {
        unreachable!()
    };
    let pathenv = std::env::var("PATH").unwrap();
    let newpath = format!("{path}/bin:{pathenv}");
    std::env::set_var("PATH", newpath);
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    let first = args[0].split('/').last().unwrap();

    match first {
        "flutterup" => {
            flutterupinfo();
        }
        "flutter" => {
            let options = &args[1..];
            match wrapper::run_wrapper("flutter", options) {
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
                                if let ProcessResult::Successed = messages {
                                    println!("Flutter is installed");
                                } else {
                                    eprintln!("Some Error");
                                }
                            }
                            Err(e) => {
                                eprintln!("SomeError : {e}");
                            }
                        }
                    }
                    constenv::ProcessResult::Successed => {}
                },
                Err(e) => {
                    eprintln!("SomethingError :{e}");
                }
            };
        }
        "dart" => {
            let options = &args[1..];
            match wrapper::run_wrapper("dart", options) {
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
                                if let ProcessResult::Successed = messages {
                                    println!("Flutter is installed");
                                } else {
                                    eprintln!("Some Error");
                                }
                            }
                            Err(e) => {
                                eprintln!("SomeError : {e}");
                            }
                        }
                    }
                    constenv::ProcessResult::Successed => {}
                },
                Err(e) => {
                    eprintln!("SomethingError :{e}");
                }
            };
        }
        _ => unimplemented!(),
    }
}
