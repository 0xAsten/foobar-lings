use std::io::Write;

use crate::args::ListArgs;
use crate::exercise::Exercise;

pub fn list(exercises: &[Exercise], subargs: &ListArgs) {
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
