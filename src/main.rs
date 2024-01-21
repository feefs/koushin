mod cli;
mod config;
mod mal;
mod spinner;

use crate::config::AuthConfig;

use clap::Parser;
use cli::{Cli, CliCommands, SetCommands};
use eyre::Result;
use owo_colors::OwoColorize;

fn koushin() -> Result<()> {
    let mut spinner = spinner::start_spinner()?;
    let auth = AuthConfig::new(&mut spinner)?;

    let c = Cli::parse();
    match &c.command {
        Some(command) => match command {
            CliCommands::List => mal::display_currently_watching_list(&auth, &mut spinner)?,
            CliCommands::Set { set_command } => {
                match set_command {
                    SetCommands::Count => mal::update_episode_count(&auth, &mut spinner, mal::EpisodeAction::Set)?,
                    SetCommands::Day => mal::update_airing_day(&auth, &mut spinner)?,
                };
                println!("{}", "更新されました!".green());
            }
            CliCommands::Mal => mal::open_my_anime_list(&auth, &mut spinner)?,
            CliCommands::Page => mal::open_anime_page(&auth, &mut spinner)?,
            CliCommands::Config { set_client } => {
                spinner::stop_spinner(&mut spinner)?;
                if *set_client {
                    config::set_client_config()?;
                } else {
                    println!("{}", config::config_folder_path()?.display());
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
