use crossterm;
use eyre::Result;
use spinners::{Spinner, Spinners};

pub fn start_spinner() -> Result<Spinner> {
    crossterm::execute!(std::io::stdout(), crossterm::cursor::SavePosition)?;
    Ok(Spinner::new(Spinners::Arc, String::new()))
}

pub fn stop_spinner(spinner: &mut Spinner) -> Result<()> {
    spinner.stop();
    crossterm::execute!(std::io::stdout(), crossterm::cursor::RestorePosition)?;
    Ok(())
}
