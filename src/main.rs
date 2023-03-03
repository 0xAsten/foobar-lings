use crate::args::{Args, Subcommands};
use crate::exercise::ExerciseList;
use crate::list::list;
use crate::lsp::lsp;
use crate::run::{find_exercise, reset, run};
use crate::verify::verify;
use crate::watch::{watch, WatchStatus};
use console::Emoji;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

#[macro_use]
mod ui;

mod args;
mod exercise;
mod list;
mod lsp;
mod project;
mod run;
mod verify;
mod watch;

// In sync with crate version
const VERSION: &'static str = "0.1.0";

fn main() {
    let args: Args = argh::from_env();

    if args.version {
        print!("v{VERSION}");
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!("\n{WELCOME}\n");
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the rustlings directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd rustlings/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("We cannot find `rustc`.");
        println!("Try running `rustc --version` to diagnose your problem.");
        println!("For instructions on how to install Rust, check the README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

    let verbose = args.nocapture;

    let command = args.nested.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });

    match command {
        Subcommands::Verify(_) => {
            verify(&exercises, (0, exercises.len()), verbose)
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Run(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            run(exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
        }
        Subcommands::Reset(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }
        Subcommands::Hint(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            println!("{}", exercise.hint);
        }
        Subcommands::List(subargs) => list(&exercises, &subargs),
        Subcommands::Lsp(_subargs) => lsp(),
        Subcommands::Watch(_subargs) => match watch(&exercises, verbose) {
            Err(e) => {
                println!(
                    "Error: Could not watch your progress. Error message was {:?}.",
                    e
                );
                println!("Most likely you've run out of disk space or your 'inotify limit' has been reached.");
                std::process::exit(1);
            }
            Ok(WatchStatus::Finished) => {
                println!(
                    "{emoji} All exercises completed! {emoji}",
                    emoji = Emoji("🎉", "★")
                );
                println!("\n{FENISH_LINE}\n");
            }
            Ok(WatchStatus::Unfinished) => {
                println!("We hope you're enjoying learning about Rust!");
                println!("If you want to continue working on the exercises at a later point, you can simply run `rustlings watch` again");
            }
        },
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}

const WELCOME: &str = r#"       welcome to...
    ____            __                  ___                 
   / __/___  ____  / /_  ____ ______   / (_)___  ____ ______
  / /_/ __ \/ __ \/ __ \/ __ `/ ___/  / / / __ \/ __ `/ ___/
 / __/ /_/ / /_/ / /_/ / /_/ / /     / / / / / / /_/ (__  ) 
/_/  \____/\____/_.___/\__,_/_/     /_/_/_/ /_/\__, /____/  
                                              /____/
"#;

const DEFAULT_OUT: &str = r#""#;

const FENISH_LINE: &str = r#"+----------------------------------------------------+
|          You made it to the Fe-nish line!          |"#;
