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

/// A String loaded from a file with AsciiDoc markup.
#[derive(Debug)]
pub struct AsciiDocText {
    pub text: String,
}

#[derive(Debug)]
enum Line<'a> {
    Para(&'a str),
    Comment(&'a str),
    Literal(&'a str),
}

impl<'a> Line<'a> {
    pub fn new(text: &'a str) -> Self {
        if text.starts_with("//") {
            Self::Comment(text)
        } else {
            Self::Para(text)
        }
    }
}

#[derive(Debug)]
pub struct ParsedAsciiDoc<'a> {
    lines: Vec<Line<'a>>,
}

impl AsciiDocText {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> ParsedAsciiDoc {
        let lines: Vec<_> = self.text.lines().map(Line::new).collect();
        ParsedAsciiDoc { lines }
    }
}
