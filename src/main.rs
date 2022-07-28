#![warn(clippy::cargo, clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

mod cli;
mod config;
mod mal;

use crate::cli::koushin;

use owo_colors::OwoColorize;

fn main() {
    std::process::exit(match koushin() {
        Ok(_) => 0,
        Err(e) => {
            println!("{} {}", "Error:".red(), e);
            1
        }
    })
}
