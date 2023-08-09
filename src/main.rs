// Enable additional clippy lints by default.
#![warn(
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::clone_on_ref_ptr,
    clippy::todo
)]
// Disable the documentation clippy lint, so that it stops suggesting backticks around AsciiDoc.
#![allow(clippy::doc_markdown)]
// Forbid unsafe code in this program.
#![forbid(unsafe_code)]

use std::fs;

use color_eyre::Result;

mod cli;
mod logging;

fn main() -> Result<()> {
    // Enable full-featured error logging.
    color_eyre::install()?;

    let args = cli::get_args();

    // Initialize the logging system based on the set verbosity
    logging::initialize_logger(args.verbose)?;

    println!("{:#?}", args);

    let file = fs::read_to_string(args.file)?;
    println!("{file}");

    Ok(())
}
