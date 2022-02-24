mod cli;
mod config;
mod error;

use crate::{cli::koushin, error::display_error};

fn main() {
    std::process::exit(match koushin() {
        Ok(_) => 0,
        Err(e) => {
            display_error(e);
            1
        }
    })
}
