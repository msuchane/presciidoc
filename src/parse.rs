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
        let mut in_comment = false;
        let mut in_literal = false;

        let mut lines = Vec::new();

        for lexed_line in &self.lines {
            match lexed_line {
                lex::Line::LineComment(text) => {
                    let parsed_line = if in_literal {
                        Line::Literal(text)
                    } else {
                        Line::Comment(text)
                    };
                    lines.push(parsed_line);
                }
                lex::Line::BlockCommentDelimeter(text) => {
                    let parsed_line = if in_literal {
                        Line::Literal(text)
                    } else {
                        in_comment = !in_comment;
                        Line::Comment(text)
                    };
                    lines.push(parsed_line);
                }
                lex::Line::LiteralDelimeter(text) => {
                    let parsed_line = if in_comment {
                        Line::Comment(text)
                    } else {
                        in_literal = !in_literal;
                        Line::Literal(text)
                    };
                    lines.push(parsed_line);
                }
                lex::Line::Text(text) => {
                    let parsed_line = if in_literal {
                        Line::Literal(text)
                    } else if in_comment {
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
