use eyre::Result;
use std::path::PathBuf;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn xdg_dirs() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix(PKG_NAME)
}

pub(super) fn config_path() -> Result<PathBuf> {
    Ok(xdg_dirs().place_config_file("config.toml")?)
}

pub(super) fn auth_path() -> Result<PathBuf> {
    Ok(xdg_dirs().place_config_file("auth.toml")?)
}

pub(super) fn config_folder_path() -> Result<PathBuf> {
    xdg_dirs().get_config_home().ok_or_else(|| eyre::eyre!("Could not find home config folder"))
}
