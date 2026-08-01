use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version)]
pub(super) struct Cli {
    #[clap(subcommand)]
    pub(super) command: Option<CliCommands>,
}

#[allow(clippy::doc_markdown)]
#[derive(Subcommand)]
pub(super) enum CliCommands {
    /// Interact with the authentication config
    Auth {
        #[clap(subcommand)]
        auth_command: Option<AuthCommands>,
    },
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
}

#[allow(clippy::doc_markdown)]
#[derive(Subcommand)]
pub(super) enum AuthCommands {
    /// Run the MyAnimeList OAuth2 authorization flow
    Login,
    /// Set MyAnimeList client ID
    SetClient,
}

#[derive(Subcommand)]
pub(super) enum SetCommands {
    /// Set episode count
    Count,
    /// Set airing day
    Day,
}
