/*
presciidoc: TODO
Copyright (C) 2023  Marek Suchánek  <marek.suchanek@protonmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

    /// Strip the file of everything except comments.
    #[bpaf(short('C'), long)]
    pub only_comments: bool,

    /// Strip the file of all code blocks (listing, source, code blocks).
    #[bpaf(short('b'), long)]
    pub no_blocks: bool,

    /// Strip the file of everything except code blocks (listing, source, code blocks).
    #[bpaf(short('B'), long)]
    pub only_blocks: bool,

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
