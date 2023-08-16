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

/// Generate pre-populated module files formatted with AsciiDoc that are used in Red Hat and Fedora documentation.
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

    /// Display debugging messages.
    #[bpaf(short, long)]
    pub verbose: bool,

    /// Process this AsciiDoc file.
    #[bpaf(positional("FILE"))]
    pub file: PathBuf,
}

/// Get command-line arguments as the `Cli` struct.
#[must_use]
pub fn get_args() -> Cli {
    cli().run()
}
