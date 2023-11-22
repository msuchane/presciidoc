/*
   Copyright 2023 Marek Suchánek

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

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
use std::io::{self, Write};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

mod asciidoc;
mod cli;
mod lex;
mod logging;
mod parse;

use asciidoc::AsciiDocText;

fn main() -> Result<()> {
    // Enable full-featured error logging.
    color_eyre::install()?;

    let args = cli::get_args();

    // Initialize the logging system based on the set verbosity
    logging::initialize_logger(args.verbose)?;

    log::debug!("Command-line arguments: {:#?}", args);

    let file = if let Some(path) = args.file {
        fs::read_to_string(path).wrap_err("Failed to read the input file.")?
    } else {
        read_stdin().wrap_err("Failed to read from standard input.")?
    };

    let adoc = AsciiDocText::new(file);
    let lexed = adoc.lex();
    let parsed = lexed.parse();
    log::debug!("{:#?}", parsed);

    let resulting_lines =
        parsed.reconsider_lines(!args.no_comments, !args.no_blocks, !args.no_paras);

    let resulting_lines = match args.output_behavior {
        cli::OutputBehavior::RemoveLines | cli::OutputBehavior::Fraction => {
            resulting_lines.into_iter().flatten().collect::<Vec<_>>()
        }
        cli::OutputBehavior::Default => resulting_lines
            .into_iter()
            .map(|line| line.unwrap_or(""))
            .collect::<Vec<_>>(),
    };

    // Prepare the output string.
    let output = match args.output_behavior {
        // In case of the "fraction" option, the output is the percentage of removed lines.
        cli::OutputBehavior::Fraction => {
            let orig_length = adoc.text.lines().count() as f32;
            let new_length = resulting_lines.len() as f32;

            // Subtracting from 1.0 so that we get the number of removed lines,
            // rather than the number of reserved lines.
            let fraction = 1.0 - (new_length / orig_length);
            // Convert to percent.
            let percentage = fraction * 100.0;

            format!("{percentage:.1}%\n")
        }
        // In case of the default output or "remove lines", prepare a modified copy of the original document.
        _ => {
            let mut resulting_document: String = resulting_lines.join("\n");

            // If the original file ended with a newline, add it to the output, too.
            // Otherwise, the previous line separation would have removed it.
            if *&adoc.text.ends_with('\n') {
                resulting_document.push('\n');
            }

            resulting_document
        }
    };

    // Print to stdout.
    safe_print(&output).wrap_err("Failed to print to standard output.")?;

    Ok(())
}

/// Read a string from stdin when `presciidoc` runs in a shell pipe.
fn read_stdin() -> Result<String> {
    let mut buffer = String::new();

    for line in io::stdin().lines() {
        buffer.push_str(&line?);
        buffer.push('\n');
    }
    Ok(buffer)
}

/// Print the final result to stdout.
/// This is a replacement for println that works around a problem with a broken pipe.
fn safe_print(text: &str) -> Result<()> {
    let status = io::stdout().write_all(text.as_bytes());
    // If the error is a "broken pipe", report it but treat it as success.
    // The operating system might use the broken pipe signal to simply stop the program.
    if let Err(e) = status {
        if e.kind() == io::ErrorKind::BrokenPipe {
            log::warn!("Broken pipe. Continuing. {e}");
        } else {
            // All other errors are real errors. Propagate them.
            return Err(e.into());
        }
    };

    Ok(())
}
