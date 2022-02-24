use crate::config::{config_folder_path, get_auth_config, set_client_config};
use crate::error::Result;

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
                    let path = config_folder_path()?
                        .into_os_string()
                        .into_string()
                        .unwrap();
                    println!("{}", path);
                }
            }
        },
        None => {
            let auth = get_auth_config()?;
            let result = ureq::get("https://api.myanimelist.net/v2/users/@me")
                .set("Authorization", &format!("Bearer {}", auth.access_token))
                .call()?
                .into_string()?;
            println!("Result: {:?}", result);
        }
    }

    // let config = get_config()?;
    // println!("Client ID: {}", config.client_id);
    // println!("Client secret: {}", config.client_secret);
    // todo!("IMPLEMENT LOGIC");
    Ok(())
}
