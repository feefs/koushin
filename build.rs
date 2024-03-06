use clap::CommandFactory;
use clap_complete::{generate_to, shells::Zsh};

mod cli {
    include!("src/cli.rs");
}

fn main() -> Result<(), std::io::Error> {
    let output_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/completions");
    generate_to(Zsh, &mut cli::Cli::command(), "koushin", output_dir)?;

    Ok(())
}
