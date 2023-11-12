#![warn(clippy::cargo, clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]
#![deny(unused_crate_dependencies)]

mod cli;
mod config;
mod mal;
mod spinner;

use crate::config::AuthConfig;

use eyre::{eyre, Result};
use owo_colors::OwoColorize;

fn koushin() -> Result<()> {
    if !cfg!(unix) {
        return Err(eyre!("Not on Unix!"));
    }

    let mut spinner = spinner::start_spinner()?;
    let auth = AuthConfig::new(&mut spinner)?;

    let c = <cli::Cli as clap::Parser>::parse();
    match &c.command {
        Some(command) => match command {
            cli::CliCommands::List => mal::display_currently_watching_list(&auth, &mut spinner)?,
            cli::CliCommands::Set { set_command } => {
                match set_command {
                    cli::SetCommands::Count => mal::update_episode_count(&auth, &mut spinner, mal::EpisodeAction::Set)?,
                    cli::SetCommands::Day => mal::update_airing_day(&auth, &mut spinner)?,
                };
                println!("{}", "更新されました!".green());
            }
            cli::CliCommands::Mal => mal::open_my_anime_list(&auth, &mut spinner)?,
            cli::CliCommands::Page => mal::open_anime_page(&auth, &mut spinner)?,
            cli::CliCommands::Config { set_client } => {
                spinner::stop_spinner(&mut spinner)?;
                if *set_client {
                    config::set_client_config()?;
                } else {
                    let Ok(path) = config::config_folder_path()?.into_os_string().into_string() else {
                        return Err(eyre!("Unable to convert OsString to String!"));
                    };
                    println!("{path}");
                }
            }
        },
        None => {
            mal::update_episode_count(&auth, &mut spinner, mal::EpisodeAction::Increment)?;
            println!("{}", "更新されました!".green());
        }
    }

    Ok(())
}

fn main() {
    std::process::exit(match koushin() {
        Ok(()) => 0,
        Err(e) => {
            println!("{} {e}", "Error:".red());
            1
        }
    })
}
