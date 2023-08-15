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
pub struct AsciiDocText<'a> {
    pub text: &'a str,
    numbered_lines: Vec<(usize, &'a str)>,
}

impl<'a> AsciiDocText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            numbered_lines: text.lines().enumerate().collect(),
        }
    }
}
