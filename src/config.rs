use crate::xdg;
use eyre::Result;
use inquire::{Password, PasswordDisplayMode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(super) struct ClientConfig {
    pub(super) client_id: String,
}

#[derive(PartialEq)]
enum ClientConfigAction {
    Set,
    Get,
}

fn check_client_config(action: &ClientConfigAction) -> Result<()> {
    let path = xdg::config_path()?;

    if !path.exists() || action == &ClientConfigAction::Set {
        let client_id = Password::new("Input MAL client ID:").with_display_mode(PasswordDisplayMode::Masked).prompt()?;
        let contents = toml::to_string_pretty(&ClientConfig { client_id })?;
        std::fs::write(&path, contents)?;
    };

    Ok(())
}

pub(super) fn get_client_config() -> Result<ClientConfig> {
    check_client_config(&ClientConfigAction::Get)?;
    let client_config = std::fs::read_to_string(xdg::config_path()?)?;

    Ok(toml::from_str(&client_config)?)
}

pub(super) fn set_client_config() -> Result<()> {
    check_client_config(&ClientConfigAction::Set)?;

    Ok(())
}
