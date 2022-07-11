use crate::config::{config_folder_path, set_client_config};
use crate::error::Result;
use crate::mal::{mal_action_prompt, mal_display_currently_watching_list, MALPromptAction};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Interact with the config
    Config {
        #[clap(short, long)]
        /// Set client config
        set_client: bool,
    },
    /// Display your currently watching anime list
    List,
    /// Set an attribute for an anime on your list
    Set {
        #[clap(subcommand)]
        set_command: SetCommands,
    },
}

#[derive(Subcommand)]
enum SetCommands {
    /// Set episode count
    Count,
    /// Set airing day
    Day,
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
            CliCommands::List => mal_display_currently_watching_list()?,
            CliCommands::Set { set_command } => {
                let action = match set_command {
                    SetCommands::Count => &MALPromptAction::SetEpisodeCount,
                    SetCommands::Day => &MALPromptAction::SetAiringDay,
                };
                mal_action_prompt(action)?
            }
        },
        None => mal_action_prompt(&MALPromptAction::IncrementEpisode)?,
    }

    Ok(())
}
