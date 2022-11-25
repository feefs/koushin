use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<CliCommands>,
}

#[derive(Subcommand)]
pub enum CliCommands {
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
pub enum SetCommands {
    /// Set episode count
    Count,
    /// Set airing day
    Day,
}
