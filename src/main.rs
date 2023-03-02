use crate::exercise::ExerciseList;
use crate::project::RustAnalyzerProject;
use crate::run::{find_exercise, reset, run};
use crate::verify::verify;
use crate::watch::{watch, WatchStatus};
use argh::FromArgs;
use console::Emoji;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;
mod watch;

// In sync with crate version
const VERSION: &'static str = "0.1.0";

#[derive(FromArgs, PartialEq, Debug)]
/// _foobar_lings is a collection of small exercises to get you used to writing and reading _foobar_ code
struct Args {
    /// show outputs from the test exercises
    #[argh(switch)]
    nocapture: bool,
    /// show the executable version
    #[argh(switch, short = 'v')]
    version: bool,
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Verify(VerifyArgs),
    Watch(WatchArgs),
    Run(RunArgs),
    Reset(ResetArgs),
    Hint(HintArgs),
    List(ListArgs),
    Lsp(LspArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "verify")]
/// Verifies all exercises according to the recommended order
struct VerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// Reruns `verify` when files were edited
struct WatchArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// Runs/Tests a single exercise
struct RunArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "reset")]
/// Resets a single exercise using "git stash -- <filename>"
struct ResetArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hint")]
/// Returns a hint for the given exercise
struct HintArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lsp")]
/// Enable rust-analyzer for exercises
struct LspArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// Lists the exercises available in Rustlings
struct ListArgs {
    #[argh(switch, short = 'p')]
    /// show only the paths of the exercises
    paths: bool,
    #[argh(switch, short = 'n')]
    /// show only the names of the exercises
    names: bool,
    #[argh(option, short = 'f')]
    /// provide a string to match exercise names
    /// comma separated patterns are acceptable
    filter: Option<String>,
    #[argh(switch, short = 'u')]
    /// display only exercises not yet solved
    unsolved: bool,
    #[argh(switch, short = 's')]
    /// display only exercises that have been solved
    solved: bool,
}

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
        Subcommands::List(subargs) => {
            if !subargs.paths && !subargs.names {
                println!("{:<17}\t{:<46}\t{:<7}", "Name", "Path", "Status");
            }
            let mut exercises_done: u16 = 0;
            let filters = subargs.filter.clone().unwrap_or_default().to_lowercase();
            exercises.iter().for_each(|e| {
                let fname = format!("{}", e.path.display());
                let filter_cond = filters
                    .split(',')
                    .filter(|f| !f.trim().is_empty())
                    .any(|f| e.name.contains(&f) || fname.contains(&f));
                let status = if e.looks_done() {
                    exercises_done += 1;
                    "Done"
                } else {
                    "Pending"
                };
                let solve_cond = {
                    (e.looks_done() && subargs.solved)
                        || (!e.looks_done() && subargs.unsolved)
                        || (!subargs.solved && !subargs.unsolved)
                };
                if solve_cond && (filter_cond || subargs.filter.is_none()) {
                    let line = if subargs.paths {
                        format!("{fname}\n")
                    } else if subargs.names {
                        format!("{}\n", e.name)
                    } else {
                        format!("{:<17}\t{fname:<46}\t{status:<7}\n", e.name)
                    };
                    // Somehow using println! leads to the binary panicking
                    // when its output is piped.
                    // So, we're handling a Broken Pipe error and exiting with 0 anyway
                    let stdout = std::io::stdout();
                    {
                        let mut handle = stdout.lock();
                        handle.write_all(line.as_bytes()).unwrap_or_else(|e| {
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                                _ => std::process::exit(1),
                            };
                        });
                    }
                }
            });
            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "Progress: You completed {} / {} exercises ({:.1} %).",
                exercises_done,
                exercises.len(),
                percentage_progress
            );
            std::process::exit(0);
        }
        Subcommands::Lsp(_subargs) => {
            let mut project = RustAnalyzerProject::new();
            project
                .get_sysroot_src()
                .expect("Couldn't find toolchain path, do you have `rustc` installed?");
            project
                .exercies_to_json()
                .expect("Couldn't parse rustlings exercises files");

            if project.crates.is_empty() {
                println!("Failed find any exercises, make sure you're in the `rustlings` folder");
            } else if project.write_to_disk().is_err() {
                println!("Failed to write rust-project.json to disk for rust-analyzer");
            } else {
                println!("Successfully generated rust-project.json");
                println!("rust-analyzer will now parse exercises, restart your language server or editor")
            }
        }
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
