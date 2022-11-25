use clap::CommandFactory;
use clap_complete::{generate_to, shells::Zsh};

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/cli.rs"));

fn main() -> Result<(), std::io::Error> {
    let output_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/gen");
    generate_to(Zsh, &mut Cli::command(), "koushin", output_dir)?;

    Ok(())
}
