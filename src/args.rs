use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// _foobar_lings is a collection of small exercises to get you used to writing and reading _foobar_ code
pub struct Args {
    /// show outputs from the test exercises
    #[argh(switch)]
    pub nocapture: bool,
    /// show the executable version
    #[argh(switch, short = 'v')]
    pub version: bool,
    #[argh(subcommand)]
    pub nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcommands {
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
pub struct VerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// Reruns `verify` when files were edited
pub struct WatchArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// Runs/Tests a single exercise
pub struct RunArgs {
    #[argh(positional)]
    /// the name of the exercise
    pub name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "reset")]
/// Resets a single exercise using "git stash -- <filename>"
pub struct ResetArgs {
    #[argh(positional)]
    /// the name of the exercise
    pub name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hint")]
/// Returns a hint for the given exercise
pub struct HintArgs {
    #[argh(positional)]
    /// the name of the exercise
    pub name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lsp")]
/// Enable rust-analyzer for exercises
pub struct LspArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// Lists the exercises available in Rustlings
pub struct ListArgs {
    #[argh(switch, short = 'p')]
    /// show only the paths of the exercises
    pub paths: bool,
    #[argh(switch, short = 'n')]
    /// show only the names of the exercises
    pub names: bool,
    #[argh(option, short = 'f')]
    /// provide a string to match exercise names
    /// comma separated patterns are acceptable
    pub filter: Option<String>,
    #[argh(switch, short = 'u')]
    /// display only exercises not yet solved
    pub unsolved: bool,
    #[argh(switch, short = 's')]
    /// display only exercises that have been solved
    pub solved: bool,
}
