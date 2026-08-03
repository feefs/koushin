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
