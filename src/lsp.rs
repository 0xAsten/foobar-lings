use crate::project::RustAnalyzerProject;

pub fn lsp() {
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
