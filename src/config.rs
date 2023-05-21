use eyre::{eyre, Result};
use inquire::{Password, PasswordDisplayMode};
use nanoid::nanoid;
use owo_colors::OwoColorize;
use qstring::QString;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tiny_http::{Response, Server};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Deserialize, Serialize)]
pub struct AuthConfig {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClientConfig {
    pub client_id: String,
}

#[derive(PartialEq)]
enum ClientConfigAction {
    Set,
    Get,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
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
        std::fs::create_dir_all(&folder_path)?;
    };

    Ok(())
}

fn check_client_config(action: &ClientConfigAction) -> Result<()> {
    setup_config_folder()?;
    let path = client_config_path()?;

    if !path.exists() || action == &ClientConfigAction::Set {
        let client_id = Password::new("Input MAL client ID:").with_display_mode(PasswordDisplayMode::Masked).prompt()?;
        let contents = toml::to_string_pretty(&ClientConfig { client_id })?;
        std::fs::write(&path, contents)?;
    };

    Ok(())
}

pub fn get_client_config() -> Result<ClientConfig> {
    check_client_config(&ClientConfigAction::Get)?;
    let client_config = std::fs::read_to_string(client_config_path()?)?;

    Ok(toml::from_str(&client_config)?)
}

pub fn set_client_config() -> Result<()> {
    check_client_config(&ClientConfigAction::Set)?;

    Ok(())
}

fn deserialize_auth_config() -> Result<AuthConfig> {
    Ok(toml::from_str(&std::fs::read_to_string(auth_config_path()?)?)?)
}

fn open_authorization() -> Result<()> {
    let config = get_client_config()?;
    let verifier = nanoid!(128);
    let challenge = verifier.clone();
    let authorization_url = format!(
        "https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id={}&code_challenge={challenge}",
        config.client_id
    );

    println!("Authorize koushin by visiting here:\n{authorization_url}\n");
    let server = match Server::http("127.0.0.1:8000") {
        Ok(s) => s,
        Err(e) => return Err(eyre!(e)),
    };

    println!("Listening for authorization code on port 8000...");

    let code_request = server.recv()?;
    let qs = QString::from(code_request.url());
    let Some(code) = qs.get("/?code") else {
        return Err(eyre!("Unable to parse code from query parameters!")) 
    };
    code_request.respond(Response::from_string("Code received!"))?;

    let token_response_json: TokenResponse = ureq::post("https://myanimelist.net/v1/oauth2/token")
        .send_form(&[
            ("client_id", &config.client_id),
            ("code", code),
            ("code_verifier", &verifier),
            ("grant_type", "authorization_code"),
        ])?
        .into_json()?;

    let path = auth_config_path()?;
    let contents = toml::to_string_pretty(&AuthConfig {
        access_token: token_response_json.access_token,
        refresh_token: token_response_json.refresh_token,
    })?;
    std::fs::write(path, contents)?;

    Ok(())
}

fn verify_refresh_auth() -> Result<()> {
    let auth_config = deserialize_auth_config()?;
    let client_config = get_client_config()?;

    if let Err(ureq::Error::Status(_, _)) =
        ureq::get("https://api.myanimelist.net/v2/users/@me").set("Authorization", &format!("Bearer {}", auth_config.access_token)).call()
    {
        let refresh_response_json: RefreshResponse = ureq::post("https://myanimelist.net/v1/oauth2/token")
            .send_form(&[
                ("client_id", &client_config.client_id),
                ("refresh_token", &auth_config.refresh_token),
                ("grant_type", "refresh_token"),
            ])?
            .into_json()?;

        let path = auth_config_path()?;
        let contents = toml::to_string_pretty(&AuthConfig {
            access_token: refresh_response_json.access_token,
            refresh_token: auth_config.refresh_token,
        })?;
        std::fs::write(path, contents)?;

        println!("{}", "Access token refreshed!".cyan());
    }

    Ok(())
}

pub fn get_auth_config() -> Result<AuthConfig> {
    setup_config_folder()?;
    let path = auth_config_path()?;
    if !path.exists() {
        open_authorization()?;
    };
    verify_refresh_auth()?;
    deserialize_auth_config()
}
