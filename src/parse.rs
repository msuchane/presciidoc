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

pub struct ParsedAsciiDoc<'a> {
    lines: Vec<Line<'a>>,
}

pub enum Line<'a> {
    Para(&'a str),
    Comment(&'a str),
    Literal(&'a str),
}

impl<'a> lex::LexedAsciiDoc<'a> {
    pub fn parse(&self) -> ParsedAsciiDoc {
        let mut lines = Vec::new();

        for lexed_line in self.lines {
            todo!()
        }

        todo!()
    }
}
