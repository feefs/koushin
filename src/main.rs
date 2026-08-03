mod agent;
mod auth;
mod cli;
mod config;
mod mal;
mod xdg;

use crate::auth::AuthConfig;
use crate::cli::{AuthCommands, Cli, CliCommands, SetCommands};
use ansi_term::Color;
use clap::Parser;
use eyre::Result;

fn koushin() -> Result<()> {
    let c = Cli::parse();

    match c.command {
        Some(CliCommands::Auth { auth_command }) => match auth_command {
            Some(AuthCommands::Login) => auth::open_authorization()?,
            Some(AuthCommands::SetClient) => config::set_client_config()?,
            None => println!("{}", xdg::config_folder_path()?.display()),
        },
        non_auth_command => {
            let auth = AuthConfig::new()?;
            match non_auth_command {
                Some(CliCommands::Auth { .. }) => unreachable!(),
                Some(CliCommands::List) => {
                    mal::display_currently_watching_list(&auth)?;
                }
                Some(CliCommands::Set { set_command }) => {
                    match set_command {
                        SetCommands::Count => mal::update_episode_count(&auth, mal::EpisodeAction::Set)?,
                        SetCommands::Day => mal::update_airing_day(&auth)?,
                    }
                    println!("{}", Color::Green.paint("更新されました!"));
                }
                Some(CliCommands::Mal) => {
                    mal::open_my_anime_list(&auth)?;
                }
                Some(CliCommands::Page) => {
                    mal::open_anime_page(&auth)?;
                }
                None => {
                    mal::update_episode_count(&auth, mal::EpisodeAction::Increment)?;
                    println!("{}", Color::Green.paint("更新されました!"));
                }
            }
        }
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
