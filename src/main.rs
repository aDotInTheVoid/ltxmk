#![deny(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

use log::{debug, error, info, trace, warn};

//mod strings;

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

    // Has it's own docs
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[structopt(short, long)]
    /// Enable timed logs
    timed_log: bool,
}

// TODO: make these configurable
const LATEX_CMD: &str = "pdflatex";
const OUTPUT_DIR: &str = "out";
const TEX_FILE_EXT: &str = "tex";

fn main() -> Result<()> {
    let mut args: Cli = Cli::from_args();

    // Set up logging
    if let Some(level) = args.verbose.log_level() {
        if !args.timed_log {
            pretty_env_logger::formatted_builder()
        } else {
            pretty_env_logger::formatted_timed_builder()
        }
        .filter(None, level.to_level_filter())
        .init();
    }

    // Clean files
    {
        let path = Path::new(OUTPUT_DIR);
        if path.is_dir() && args.clean {
            info!("Removing dir {}", OUTPUT_DIR);
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
        info!("No files given, using all");
        for file in fs::read_dir(std::env::current_dir()?)? {
            let file = file?.path();
            if file.is_file() && file.extension() == Some(std::ffi::OsStr::new(TEX_FILE_EXT)) {
                args.files.push(file);
            }
        }
    }

    trace!("Files to process: {:#?}", &args.files);

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

    info!("Creating dir: {}", &output_dir);
    fs::create_dir_all(&output_dir)?;

    //TODO: Figure out how many times to run
    for _ in 0..3 {
        debug!(
            "Running {} -recorder -output-directory={} -halt-on-error {}",
            LATEX_CMD, output_dir, output_file,
        );
        let output = Command::new(LATEX_CMD)
            .arg("-recorder")
            .arg(format!("-output-directory={}", output_dir))
            .arg("-halt-on-error")
            .arg(file)
            .output()?;
    }
    //println!("{}", std::str::from_utf8(&output.stdout)?);

    info!("Moving {} to {}", generated_file, output_file);
    fs::rename(generated_file, output_file)?;

    Ok(())
}
