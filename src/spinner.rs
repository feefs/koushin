use crossterm::{
    cursor::{RestorePosition, SavePosition},
    ExecutableCommand,
};
use eyre::Result;
use spinners::{Spinner, Spinners};
use std::io;

pub(crate) fn start_spinner() -> Result<Spinner> {
    io::stdout().execute(SavePosition)?;
    Ok(Spinner::new(Spinners::Arc, String::new()))
}

pub(crate) fn stop_spinner(spinner: &mut Spinner) -> Result<()> {
    spinner.stop();
    io::stdout().execute(RestorePosition)?;
    Ok(())
}
