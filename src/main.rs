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

    println!("{:#?}", args);

    let file = fs::read_to_string(args.file)?;
    let adoc = AsciiDocText::new(file);
    let lexed = adoc.lex();
    let parsed = lexed.parse();
    println!("{:#?}", parsed);

    Ok(())
}
