#![deny(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

mod strings;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The files to process
    files: Vec<PathBuf>,
    // #[structopt(long, short)]
    // /// Enables continuous compilation
    // continuous: bool,
    #[structopt(long, short)]
    /// Enables cleaning the output dir before compilation
    clean: bool,
}

// TODO: make these configurable
const LATEX_CMD: &str = "pdflatex";
const OUTPUT_DIR: &str = "out";
const TEX_FILE_EXT: &str = "tex";

fn main() -> Result<()> {
    let mut args: Cli = Cli::from_args();

    // Clean files
    {
        let path = Path::new(OUTPUT_DIR);
        if path.is_dir() && args.clean {
            fs::remove_dir_all(OUTPUT_DIR)?;
        } else if path.is_file() {
            return Err(anyhow!(
                "{} is a file, but we need it to be a directory",
                OUTPUT_DIR
            ));
        }
    }

    // Populate args if empty
    // If no files given, build every tex file
    if args.files.is_empty() {
        for file in fs::read_dir(std::env::current_dir()?)? {
            let file = file?.path();
            if file.is_file() && file.extension() == Some(std::ffi::OsStr::new(TEX_FILE_EXT)) {
                args.files.push(file);
            }
        }
    }

    // Process each arg
    for i in args.files {
        process_file(&i)?;
    }

    Ok(())
}

fn process_file(file: &Path) -> Result<()> {
    if !(file.is_file()) {
        return Err(anyhow!("File does not exist: {:?}", &file));
    }

    let stem = file
        .file_stem()
        .with_context(|| format!("Invalid file name: {:?}", &file))?
        .to_str()
        .with_context(|| format!("Invalid file name (not utf8): {:?}", &file))?;

    //TODO: Use path operators, not strings
    //      I dont know if this works on windows

    // Eg out/Chemistry
    let output_dir = format!("{}/{}", OUTPUT_DIR, stem);
    // Eg out/Chemistry/Chemistry.pdf
    let generated_file = format!("{}/{}/{}.pdf", OUTPUT_DIR, stem, stem);
    // Eg: out/Chemistry.pdf
    let output_file = format!("{}/{}.pdf", OUTPUT_DIR, stem);

    fs::create_dir_all(&output_dir)?;

    //TODO: Figure out how many times to run
    for _ in 0..3 {
        let output = Command::new(LATEX_CMD)
            .arg("-recorder")
            .arg(format!("-output-directory={}", output_dir))
            .arg("-halt-on-error")
            .arg(file)
            .output()?;
    }
    //println!("{}", std::str::from_utf8(&output.stdout)?);

    fs::rename(generated_file, output_file)?;

    Ok(())
}
