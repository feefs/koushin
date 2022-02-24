use owo_colors::OwoColorize;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn display_error(err: Box<dyn Error>) {
    println!("{} {}", "Error:".red(), err);
}
