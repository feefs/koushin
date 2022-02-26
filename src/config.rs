use crate::error::Result;

use inquire::{Password, PasswordDisplayMode};
use nanoid::nanoid;
use qstring::QString;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tiny_http::{Response, Server};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Deserialize, Serialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(PartialEq)]
enum ClientConfigAction {
    Set,
    Get,
}

#[derive(Deserialize, Serialize)]
pub struct AuthConfig {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

pub fn config_folder_path() -> Result<PathBuf> {
    Ok(std::path::PathBuf::from(std::env::var("HOME")?).join(".config").join(PKG_NAME))
}

fn client_config_path() -> Result<PathBuf> {
    Ok(config_folder_path()?.join("config.toml"))
}

fn auth_config_path() -> Result<PathBuf> {
    Ok(config_folder_path()?.join("auth.toml"))
}

fn setup_config_folder() -> Result<()> {
    let folder_path = config_folder_path()?;
    if !folder_path.exists() {
        std::fs::create_dir_all(&folder_path)?
    };
    Ok(())
}

fn check_client_config(action: ClientConfigAction) -> Result<()> {
    setup_config_folder()?;
    let path = client_config_path()?;
    if !path.exists() || action == ClientConfigAction::Set {
        let client_id = Password::new("Input MAL client ID:").with_display_mode(PasswordDisplayMode::Masked).prompt()?;
        let client_secret = Password::new("Input MAL client secret:").with_display_mode(PasswordDisplayMode::Masked).prompt()?;
        std::fs::write(&path, toml::to_string_pretty(&ClientConfig { client_id, client_secret }).unwrap())?
    };
    Ok(())
}

pub fn get_client_config() -> Result<ClientConfig> {
    check_client_config(ClientConfigAction::Get)?;
    let client_config = std::fs::read(client_config_path()?)?;
    Ok(toml::from_slice(&client_config)?)
}

pub fn set_client_config() -> Result<()> {
    check_client_config(ClientConfigAction::Set)?;
    Ok(())
}

fn open_authorization() -> Result<()> {
    let config = get_client_config()?;
    let verifier = nanoid!(128);
    let challenge = verifier.clone();
    let authorization_url = format!(
        "https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id={}&code_challenge={}",
        config.client_id, challenge
    );
    println!("Authorize koushin by visiting here:\n{}\n", authorization_url);
    let server = Server::http("127.0.0.1:8000").unwrap();
    println!("Listening for authorization code on port 8000...");

    let code_request = server.recv()?;
    let qs = QString::from(code_request.url());
    let code = qs.get("/?code").unwrap();
    code_request.respond(Response::from_string("Code received!"))?;

    let token_response_json: TokenResponse = ureq::post("https://myanimelist.net/v1/oauth2/token")
        .send_form(&[
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("code", code),
            ("code_verifier", &verifier),
            ("grant_type", "authorization_code"),
        ])?
        .into_json()?;

    std::fs::write(
        &auth_config_path()?,
        toml::to_string_pretty(&AuthConfig {
            access_token: token_response_json.access_token,
            refresh_token: token_response_json.refresh_token,
        })
        .unwrap(),
    )?;
    Ok(())
}

pub fn get_auth_config() -> Result<AuthConfig> {
    setup_config_folder()?;
    let path = auth_config_path()?;
    if !path.exists() {
        open_authorization()?;
    }
    let auth_config = std::fs::read(auth_config_path()?)?;
    Ok(toml::from_slice(&auth_config)?)
}
