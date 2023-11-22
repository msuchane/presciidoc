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

//! # `cli.rs`
//!
//! This module defines the command-line arguments and behavior.

use std::path::PathBuf;

use bpaf::Bpaf;

/// Preprocessing AsciiDoc for other tools.
#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
pub struct Cli {
    /// Strip the file of all comments.
    #[bpaf(short('c'), long)]
    pub no_comments: bool,

    /// Strip the file of all code blocks (listing, source, code blocks).
    #[bpaf(short('b'), long)]
    pub no_blocks: bool,

    /// Strip the file of all items that are neither comments nor code, such as paragraphs.
    #[bpaf(short('p'), long)]
    pub no_paras: bool,

    #[bpaf(external, fallback(OutputBehavior::Default))]
    pub output_behavior: OutputBehavior,

    /// Display debugging messages.
    #[bpaf(short, long)]
    pub verbose: bool,

    /// Process this AsciiDoc file. Otherwise, the program reads from standard input.
    #[bpaf(positional("FILE"))]
    pub file: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Bpaf)]
pub enum OutputBehavior {
    /// Generate the file without any comments.
    /// This option is now the default.
    /// The option is hidden, has no effect, and exists only for compatibility
    /// with previous releases.
    /// Remove the selected lines rather than replacing them with blank lines.
    #[bpaf(short('r'), long)]
    RemoveLines,
    /// Instead of the modified file content, only calculate what percentage
    /// of the lines in the file would get removed by this command.
    /// In other words, this tells you the percentage of comments, literals, etc. in the file.
    #[bpaf(short('f'), long)]
    Fraction,
    #[bpaf(hide)]
    Default,
}

/// Get command-line arguments as the `Cli` struct.
#[must_use]
pub fn get_args() -> Cli {
    cli().run()
}
