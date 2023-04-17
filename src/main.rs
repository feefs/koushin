#![warn(clippy::cargo, clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

mod cli;
mod config;
mod mal;

use eyre::{eyre, Result};
use owo_colors::OwoColorize;

fn koushin() -> Result<()> {
    if !cfg!(unix) {
        return Err(eyre!("Not on Unix!"));
    }

    let c = <cli::Cli as clap::Parser>::parse();
    match &c.command {
        Some(command) => match command {
            cli::CliCommands::List => mal::display_currently_watching_list()?,
            cli::CliCommands::Set { set_command } => {
                match set_command {
                    cli::SetCommands::Count => mal::update_episode_count(mal::EpisodeAction::Set)?,
                    cli::SetCommands::Day => mal::update_airing_day()?,
                };
                println!("{}", "更新されました!".green());
            }
            cli::CliCommands::Mal => mal::open_my_anime_list()?,
            cli::CliCommands::Page => mal::open_anime_page()?,
            cli::CliCommands::Config { set_client } => {
                if *set_client {
                    config::set_client_config()?;
                } else {
                    let Ok(path) = config::config_folder_path()?.into_os_string().into_string() else {
                        return Err(eyre!("Unable to convert OsString to String!"))
                    };
                    println!("{path}");
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

fn main() {
    std::process::exit(match koushin() {
        Ok(_) => 0,
        Err(e) => {
            println!("{} {e}", "Error:".red());
            1
        }
    })
}
