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

use color_eyre::eyre::{Result, WrapErr};
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

/// This function initializes the `simplelog` logging system, which plugs into the `log`
/// infrastructure. The function returns nothing. It only affects the global state when it runs.
pub fn initialize_logger(verbose: bool) -> Result<()> {
    // Set the verbosity level based on the command-line options.
    let verbosity = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    let config = ConfigBuilder::new()
        // Display a time stamp only for the most verbose level.
        .set_time_level(LevelFilter::Trace)
        // Display the thread number only for the most verbose level.
        .set_thread_level(LevelFilter::Trace)
        .build();

    TermLogger::init(
        verbosity,
        config,
        // Mixed mode prints errors to stderr and info to stdout. Not sure about the other levels.
        TerminalMode::Mixed,
        // Try to use color if possible.
        ColorChoice::Auto,
    )
    .wrap_err("Failed to configure the terminal logging.")?;

    Ok(())
}
