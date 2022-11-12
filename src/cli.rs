use crate::{config, mal};

use clap::{Parser, Subcommand};
use eyre::{eyre, Result};
use owo_colors::OwoColorize;

#[derive(Parser)]
#[clap(about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Display your MyAnimeList in an organized format
    List,
    /// Set an attribute for an anime
    Set {
        #[clap(subcommand)]
        set_command: SetCommands,
    },
    /// Open your MyAnimeList in the browser
    Mal,
    /// Open the page for an anime in the browser
    Page,
    /// Interact with the config
    Config {
        #[clap(short, long)]
        /// Set client config
        set_client: bool,
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
        return Err(eyre!("Not on Unix!"));
    }

    let cli = Cli::parse();
    match &cli.command {
        Some(command) => match command {
            CliCommands::List => mal::display_currently_watching_list()?,
            CliCommands::Set { set_command } => {
                match set_command {
                    SetCommands::Count => mal::update_episode_count(mal::EpisodeAction::Set)?,
                    SetCommands::Day => mal::update_airing_day()?,
                };
                println!("{}", "更新されました!".green());
            }
            CliCommands::Mal => mal::open_my_anime_list()?,
            CliCommands::Page => mal::open_anime_page()?,
            CliCommands::Config { set_client } => {
                if *set_client {
                    config::set_client_config()?;
                } else {
                    let path = match config::config_folder_path()?.into_os_string().into_string() {
                        Ok(p) => p,
                        Err(_) => return Err(eyre!("Unable to convert OsString to String!")),
                    };
                    println!("{}", path);
                }
            }
        },
        None => {
            mal::update_episode_count(mal::EpisodeAction::Increment)?;
            println!("{}", "更新されました!".green());
        }
    }

    Ok(())
}
