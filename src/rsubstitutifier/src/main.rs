use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod mock_attribute;
mod rewriter;

#[derive(Parser, Debug)]
#[command(
    about = "Applies #[rsubstitute::mock(base)] on all applicable items in given Rust source code directory."
)]
#[command(long_about = None)]
struct Args {
    source_code_dir_path: PathBuf,
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args = Args::parse();

    println!(
        "Modifying source code in directory '{:?}'...",
        args.source_code_dir_path.to_str()
    );

    let mut processed_files_count = 0;
    let mut valid_structs = HashSet::new();
    for maybe_dir_entry in WalkDir::new(args.source_code_dir_path) {
        let dir_entry = maybe_dir_entry?;
        let path = dir_entry.path();
        if path.extension().is_some_and(|s| s == "rs") {
            process_file(path, &mut valid_structs)?;
            processed_files_count += 1;
        }
    }

    println!("Done, processed {processed_files_count} file(s).");
    Ok(())
}

fn process_file(
    path: &Path,
    valid_structs: &mut HashSet<String>,
) -> Result<(), Box<dyn core::error::Error>> {
    let src = fs::read_to_string(path)?;
    let mut file: syn::File = syn::parse_file(&src)?;

    rewriter::rewrite(&mut file, valid_structs);
    let formatted = prettyplease::unparse(&file);
    fs::write(path, formatted)?;

    Ok(())
}
