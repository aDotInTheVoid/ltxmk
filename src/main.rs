#![deny(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use anyhow::{anyhow, Context, Result};
use regex::bytes::Regex;
use structopt::StructOpt;
use lazy_static::lazy_static;

use log::{debug, error, info, trace, warn};

//mod strings;
//mod comp_db;
//mod fls;
//use crate::fls::FLSFile;


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
const MAX_LOG_LEN: usize = 1000;

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
    let generated_file = format!("{}/{}.pdf", output_dir, stem);
    // Eg: out/Chemistry.pdf
    let output_file = format!("{}/{}.pdf", OUTPUT_DIR, stem);
    // Eg: out/Chemistry/Chemistry.fls
    //let fls_file: PathBuf = format!("{}/{}.fls", output_dir, stem).into();

    // FLSFile is just pointers to another string
    // Declare the buffer here that it will live long enough
    // We also need to give it something, as rust wont allow a reference to uninitialized reference
    // even if it will always be overridden.
    // This is cheep, as String::new doesn't allocate.
    //let mut fls_buffer = std::io::Result::Ok(String::new());
    //let fls = read_fls(fls_file, &mut fls_buffer);

    info!("Creating dir: {}", &output_dir);
    fs::create_dir_all(&output_dir)?;

    let mut output = run_pdflatex(file, &output_dir).with_context(|| "Failed to invoke latex")?;

    // TODO: Figure out exit status
    handle_output_err(&output)?;

    while run_again(&output) {
        output = run_pdflatex(file, &output_dir).with_context(|| "Failed to invoke latex")?;
        handle_output_err(&output)?;
    }

    info!("Moving {} to {}", generated_file, output_file);
    fs::rename(generated_file, output_file)?;

    Ok(())
}

fn handle_output_err(output: &Output) -> Result<()> {
    if !output.status.success() {
        error!(
            "{} failed to run, exit code {}",
            LATEX_CMD,
            output
                .status
                .code()
                .map(|x| x.to_string())
                .unwrap_or_else(|| "unknown".to_string()) // TODO: avoid unnecessary allocation
        );
        error!(
            "{}",
            String::from_utf8_lossy(
                &output.stdout[output.stdout.len().saturating_sub(MAX_LOG_LEN)..]
            )
        );

        Err(anyhow!("{} exited in failure", LATEX_CMD))
    } else {
        Ok(())
    }
}

fn run_pdflatex(file: &Path, output_dir: &str) -> std::result::Result<Output, std::io::Error> {
    // TODO: Make the debug and Command be the same
    debug!(
        "Running `{} -recorder -output-directory={} -halt-on-error {}`",
        LATEX_CMD,
        output_dir,
        file.as_os_str().to_str().unwrap_or_else(|| {
            warn!("Invalid utf8");
            "<invalid utf8>"
        }),
    );
    Command::new(LATEX_CMD)
        .arg("-recorder")
        .arg(format!("-output-directory={}", output_dir))
        .arg("-halt-on-error")
        .arg(file)
        .output()
}


// TODO: Make this smarter
// Clippy doesn't know their is no way to see if a [u8] contains a [u8] in std
// Also regex has literal optimizations
// Also it'l be easier when this is more complex
#[allow(clippy::trivial_regex)]
fn run_again(output: &Output) -> bool {
    lazy_static! {

        static ref MATCHES: Regex = Regex::new("Rerun").unwrap();
    }

    // TODO: Cache regex
    MATCHES.is_match(&output.stdout)
}
