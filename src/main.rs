use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use cr_prep::{is_target_file, process_file};

/// A CLI tool for collecting code files for code review
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to search for code files
    #[arg(short, long)]
    path: PathBuf,

    /// Output file path (outputs to stdout if not specified)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn run() -> Result<()> {
    let args = Args::parse();

    if !args.path.is_dir() {
        anyhow::bail!("Specified path is not a directory: {}", args.path.display());
    }

    let mut output = String::new();
    for entry in WalkDir::new(&args.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_owned();
        if path.is_file() && is_target_file(&path) {
            match process_file(&path, &args.path) {
                Ok(content) => output.push_str(&content),
                Err(err) => eprintln!("Warning: {}", err),
            }
        }
    }

    match args.output {
        Some(output_path) => {
            let mut file = File::create(&output_path)
                .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
            file.write_all(output.as_bytes())
                .with_context(|| format!("Failed to write to output file: {}", output_path.display()))?;
        }
        None => {
            io::stdout()
                .write_all(output.as_bytes())
                .context("Failed to write to stdout")?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
