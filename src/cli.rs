use crate::config::{config_folder_path, set_client_config};
use crate::error::Result;
use crate::mal::{mal_prompt, MALPromptAction};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Option<CliCommands>,
    #[clap(short, long)]
    /// Set specific episode count instead of incrementing
    set_specific: bool,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Interact with the config
    Config {
        #[clap(long)]
        /// Set client config
        set_client: bool,
    },
}

pub fn koushin() -> Result<()> {
    if !cfg!(unix) {
        return Err("Not on Unix!".into());
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            CliCommands::Config { set_client } => {
                if *set_client {
                    set_client_config()?;
                } else {
                    let path = config_folder_path()?.into_os_string().into_string().unwrap();
                    println!("{}", path);
                }
            }
        },
        None => {
            mal_prompt(if cli.set_specific {
                MALPromptAction::Set
            } else {
                MALPromptAction::Increment
            })?;
        }
    }
    Ok(())
}
