mod auth;
mod cli;
mod config;
mod mal;
mod spinner;
mod xdg;

use crate::auth::AuthConfig;
use crate::cli::{Cli, CliCommands, SetCommands};
use ansi_term::Color;
use clap::Parser;
use eyre::Result;

fn koushin() -> Result<()> {
    let auth = AuthConfig::new()?;

    let c = Cli::parse();
    if let Some(command) = &c.command {
        match command {
            CliCommands::List => mal::display_currently_watching_list(&auth)?,
            CliCommands::Set { set_command } => {
                match set_command {
                    SetCommands::Count => mal::update_episode_count(&auth, mal::EpisodeAction::Set)?,
                    SetCommands::Day => mal::update_airing_day(&auth)?,
                }
                println!("{}", Color::Green.paint("更新されました!"));
            }
            CliCommands::Mal => mal::open_my_anime_list(&auth)?,
            CliCommands::Page => mal::open_anime_page(&auth)?,
            CliCommands::Config { set_client } => {
                if *set_client {
                    config::set_client_config()?;
                } else {
                    println!("{}", xdg::config_folder_path()?.display());
                }
            }
        }
    } else {
        mal::update_episode_count(&auth, mal::EpisodeAction::Increment)?;
        println!("{}", Color::Green.paint("更新されました!"));
    }

    Ok(())
}

fn main() {
    std::process::exit(match koushin() {
        Ok(()) => 0,
        Err(e) => {
            println!("{} {e}", Color::Red.paint("Error:"));
            1
        }
    })
}
