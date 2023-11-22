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

use crate::lex;

#[derive(Debug)]
pub struct ParsedAsciiDoc<'a> {
    pub lines: Vec<Line<'a>>,
}

#[derive(Debug)]
pub enum Line<'a> {
    Para(&'a str),
    Comment(&'a str),
    Literal(&'a str),
}

impl<'a> lex::LexedAsciiDoc<'a> {
    pub fn parse(&self) -> ParsedAsciiDoc {
        // These values act as "extra bools": They store a truth value (are we inside a block?)
        // and they also store the exact delimiter that opened the block, so that we can
        // match the same ending delimiter later.
        let mut in_comment = None;
        let mut in_literal = None;

        // This vector stores the resulting, parsed lines.
        let mut lines = Vec::new();

        for lexed_line in &self.lines {
            match lexed_line {
                // Parse line comments.
                lex::Line::LineComment(text) => {
                    let parsed_line = if in_literal.is_some() {
                        Line::Literal(text)
                    } else {
                        Line::Comment(text)
                    };
                    lines.push(parsed_line);
                }
                // Parse block comments.
                lex::Line::BlockCommentDelimeter(text) => {
                    // If the line appears as a block comment, but we're in a literal block,
                    // the line is in fact a literal.
                    let parsed_line = if in_literal.is_some() {
                        Line::Literal(text)
                    // If we're inside a comment block...
                    } else if let Some(delimiter) = in_comment {
                        // If the block delimiters match, close the comment block here.
                        // TODO: Find out if the delimiter *lines* must always be identical,
                        // or if one of them can have whitespace or some other stuff at the end
                        // and still match as closing.
                        if delimiter == text {
                            in_comment = None;
                        }
                        // Up to and including the closing delimiter, everything is a comment block.
                        Line::Comment(text)
                    // If we're not inside any block...
                    } else {
                        // Open a comment block here.
                        in_comment = Some(text);
                        Line::Comment(text)
                    };
                    lines.push(parsed_line);
                }
                // Parse literal blocks. See the comments on the `BlockCommentDelimeter` parsing.
                lex::Line::LiteralDelimeter(text) => {
                    let parsed_line = if in_comment.is_some() {
                        Line::Comment(text)
                    } else if let Some(delimiter) = in_literal {
                        if delimiter == text {
                            in_literal = None;
                        }
                        Line::Literal(text)
                    } else {
                        in_literal = Some(text);
                        Line::Literal(text)
                    };
                    lines.push(parsed_line);
                }
                // Parse other text. This is any element that wasn't lexed as any particular kind before.
                lex::Line::Text(text) => {
                    let parsed_line = if in_literal.is_some() {
                        Line::Literal(text)
                    } else if in_comment.is_some() {
                        Line::Comment(text)
                    } else {
                        Line::Para(text)
                    };
                    lines.push(parsed_line);
                }
            }
        }

        ParsedAsciiDoc { lines }
    }
}

impl<'a> ParsedAsciiDoc<'a> {
    pub fn reconsider_lines(
        self,
        comments: bool,
        literals: bool,
        paras: bool,
    ) -> Vec<Option<&'a str>> {
        self.lines
            .into_iter()
            .map(|line| match line {
                Line::Comment(text) => {
                    if comments {
                        Some(text)
                    } else {
                        None
                    }
                }
                Line::Literal(text) => {
                    if literals {
                        Some(text)
                    } else {
                        None
                    }
                }
                Line::Para(text) => {
                    if paras {
                        Some(text)
                    } else {
                        None
                    }
                }
            })
            .collect()
    }
}
