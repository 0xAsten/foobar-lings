use serde::Deserialize;
use std::path::PathBuf;

// The mode of the exercise.
#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    // Indicates that the exercise should be compiled as a binary
    Compile,
    // Indicates that the exercise should be compiled as a test harness
    Test,
    // Indicates that the exercise should be linted with clippy
    Clippy,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

// A representation of a exercise.
// This is deserialized from the accompanying info.toml file
#[derive(Deserialize, Debug)]
pub struct Exercise {
    // Name of the exercise
    pub name: String,
    // The path to the file containing the exercise's source code
    pub path: PathBuf,
    // The mode of the exercise (Test, Compile, or Clippy)
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
}
