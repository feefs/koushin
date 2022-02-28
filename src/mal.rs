use crate::config::{get_auth_config, AuthConfig};
use crate::error::Result;

use chrono::{DateTime, FixedOffset};
use inquire::{Confirm, Select};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AnimeListResponse {
    paging: Paging,
    data: Vec<Data>,
}

#[derive(Deserialize, Serialize)]
struct Paging {
    next: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Data {
    node: Node,
}

#[derive(Deserialize, Serialize)]
struct Node {
    title: String,
    id: usize,
    num_episodes: usize,
    my_list_status: Status,
}

#[derive(Deserialize, Serialize)]
struct Status {
    num_episodes_watched: usize,
    updated_at: String,
}

#[derive(Debug)]
struct Entry {
    title: String,
    id: usize,
    watched_episodes: usize,
    total_episodes: usize,
    last_updated: DateTime<FixedOffset>,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.watched_episodes.to_string().cyan(), self.title)
    }
}

fn get_entries(auth: &AuthConfig) -> Result<Vec<Entry>> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut page: AnimeListResponse =
        ureq::get("https://api.myanimelist.net/v2/users/@me/animelist?status=watching&fields=my_list_status,num_episodes")
            .set("Authorization", &format!("Bearer {}", auth.access_token))
            .call()?
            .into_json()?;
    loop {
        for data in &page.data {
            entries.push(Entry {
                title: data.node.title.to_owned(),
                id: data.node.id,
                watched_episodes: data.node.my_list_status.num_episodes_watched,
                total_episodes: data.node.num_episodes,
                last_updated: DateTime::parse_from_rfc3339(&data.node.my_list_status.updated_at).unwrap(),
            })
        }
        if page.paging.next.is_none() {
            break;
        }
        page = ureq::get(&page.paging.next.unwrap()).set("Authorization", &format!("Bearer {}", auth.access_token)).call()?.into_json()?;
    }
    entries.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));
    Ok(entries)
}

fn update_entry(auth: &AuthConfig, entry: &Entry) -> Result<()> {
    ureq::patch(&format!("https://api.myanimelist.net/v2/anime/{}/my_list_status", entry.id))
        .set("Authorization", &format!("Bearer {}", auth.access_token))
        .send_form(&[("num_watched_episodes", (entry.watched_episodes + 1).to_string().as_str())])?;
    println!("{}", "更新されました!".green());
    Ok(())
}

pub fn mal_prompt() -> Result<()> {
    let auth = get_auth_config()?;
    let entries = get_entries(&auth)?;
    let entry = Select::new("Select an anime you are currently watching:", entries).prompt()?;
    let ans = Confirm::new(&format!("Update \"{}\"?", entry.title))
        .with_default(true)
        .with_help_message(&format!(
            "{} -> {}/{} episodes",
            entry.watched_episodes,
            entry.watched_episodes + 1,
            entry.total_episodes
        ))
        .prompt()?;
    if ans {
        update_entry(&auth, &entry)?;
    }
    Ok(())
}
