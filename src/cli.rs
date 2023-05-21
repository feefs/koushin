use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<CliCommands>,
}

#[derive(Subcommand)]
pub enum CliCommands {
    /// Display your MyAnimeList in an organized format [alias: l]
    #[command(alias = "l")]
    List,
    /// Set an attribute for an anime                   [alias: s]
    #[command(alias = "s")]
    Set {
        #[clap(subcommand)]
        set_command: SetCommands,
    },
    /// Open your MyAnimeList in the browser            [alias: m]
    #[command(alias = "m")]
    Mal,
    /// Open the page for an anime in the browser       [alias: p]
    #[command(alias = "p")]
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
