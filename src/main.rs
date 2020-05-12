#![deny(unsafe_code)]

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use std::process::Command;
use structopt::StructOpt;

mod strings;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The files to process
    files: Vec<PathBuf>,
    #[structopt(long, short)]
    /// Enables continuous compilation
    continuous: bool,
}

// TODO: make these configurable
const LATEX_CMD: &str = "pdflatex";
const OUTPUT_DIR: &str = "out";

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    for i in args.files {
        process_file(&i)?;
    }

    Ok(())
}

fn process_file(file: &Path) -> Result<()> {
    if !(file.is_file()) {
        return Err(anyhow!("File does not exist: {:?}", &file));
    }

    let output_dir = format!(
        "{}/{}",
        OUTPUT_DIR,
        file.file_stem()
            .with_context(|| format!("Invalid file name: {:?}", &file))?
            .to_str()
            .with_context(|| format!("Invalid file name (not utf8): {:?}", &file))?
    );

    std::fs::create_dir_all(&output_dir)?;

    let output = Command::new(LATEX_CMD)
        .arg("-recorder")
        .arg(format!("-output-directory={}", output_dir))
        .arg("-halt-on-error")
        .arg(file)
        .output()?;

    println!("{}", std::str::from_utf8(&output.stderr)?);
    Ok(())
}
