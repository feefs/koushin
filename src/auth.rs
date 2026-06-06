use crate::config;
use crate::spinner;
use crate::xdg;
use ansi_term::Color;
use eyre::{Result, eyre};
use nanoid::nanoid;
use qstring::QString;
use serde::{Deserialize, Serialize};
use tiny_http::{Response, Server};

#[derive(Deserialize, Serialize)]
pub(super) struct AuthConfig {
    pub(super) access_token: String,
    pub(super) refresh_token: String,
}

impl AuthConfig {
    pub(super) fn new() -> Result<Self> {
        let auth_path = xdg::auth_path()?;
        if !auth_path.exists() {
            open_authorization()?;
        }
        verify_refresh_auth()?;
        deserialize_auth_config()
    }
}

fn deserialize_auth_config() -> Result<AuthConfig> {
    Ok(toml::from_str(&std::fs::read_to_string(xdg::auth_path()?)?)?)
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

fn open_authorization() -> Result<()> {
    let config = config::get_client_config()?;
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
        return Err(eyre!("Unable to parse code from query parameters!"));
    };
    code_request.respond(Response::from_string("Code received!"))?;

    let token_response_json: TokenResponse = spinner::agent()
        .post("https://myanimelist.net/v1/oauth2/token")
        .send_form([
            ("client_id", config.client_id.as_str()),
            ("code", code),
            ("code_verifier", verifier.as_str()),
            ("grant_type", "authorization_code"),
        ])?
        .into_body()
        .read_json()?;

    let path = xdg::auth_path()?;
    let contents = toml::to_string_pretty(&AuthConfig {
        access_token: token_response_json.access_token,
        refresh_token: token_response_json.refresh_token,
    })?;
    std::fs::write(path, contents)?;

    Ok(())
}

fn verify_refresh_auth() -> Result<()> {
    let auth_config = deserialize_auth_config()?;
    let client_config = config::get_client_config()?;

    let res = spinner::agent()
        .get("https://api.myanimelist.net/v2/users/@me")
        .header("Authorization", format!("Bearer {}", auth_config.access_token))
        .call()?;

    if !res.status().is_success() {
        let refresh_response_json: RefreshResponse = spinner::agent()
            .post("https://myanimelist.net/v1/oauth2/token")
            .send_form([
                ("client_id", client_config.client_id.as_str()),
                ("refresh_token", auth_config.refresh_token.as_str()),
                ("grant_type", "refresh_token"),
            ])?
            .into_body()
            .read_json()?;

        let path = xdg::auth_path()?;
        let contents = toml::to_string_pretty(&AuthConfig {
            access_token: refresh_response_json.access_token,
            refresh_token: auth_config.refresh_token,
        })?;
        std::fs::write(path, contents)?;

        println!("{}", Color::Cyan.paint("Access token refreshed!"));
    }

    Ok(())
}
