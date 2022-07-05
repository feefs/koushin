use crate::config::{config_folder_path, set_client_config};
use crate::error::Result;
use crate::mal::{mal_action_prompt, mal_display_currently_watching_list, MALPromptAction};

use clap::{ArgGroup, Parser, Subcommand};

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
    #[clap(group(ArgGroup::new("set").required(true)))]
    /// Set an attribute for an anime on your list
    Set {
        #[clap(group = "set", short = 'e', long = "episode")]
        /// Set episode count
        set_episode: bool,
        #[clap(group = "set", short = 'd', long = "day")]
        /// Set airing day
        set_airing_day: bool,
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
            CliCommands::List => mal_display_currently_watching_list()?,
            CliCommands::Set { set_episode, set_airing_day } => {
                let action = match (set_episode, set_airing_day) {
                    (true, _) => &MALPromptAction::SetEpisode,
                    (_, true) => &MALPromptAction::SetAiringDay,
                    (false, false) => unreachable!(),
                };
                mal_action_prompt(action)?
            }
        },
        None => mal_action_prompt(&MALPromptAction::IncrementEpisode)?,
    }

    Ok(())
}
