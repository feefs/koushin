use crate::config::AuthConfig;
use crate::spinner;

use chrono::{Datelike, Local, Weekday};
use eyre::Result;
use inquire::{formatter::OptionFormatter, Confirm, CustomType, Select, Text};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const MAX_ENTRY_TITLE_LENGTH: usize = 88;

static WEEKDAY_MAPPINGS: phf::Map<&'static str, Weekday> = phf::phf_map! {
    "monday" => Weekday::Mon,
    "tuesday" => Weekday::Tue,
    "wednesday" => Weekday::Wed,
    "thursday" => Weekday::Thu,
    "friday" => Weekday::Fri,
    "saturday" => Weekday::Sat,
    "sunday" => Weekday::Sun,
};

static AIRING_DAY_MAPPINGS: phf::Map<u32, &'static str> = phf::phf_map! {
    0_u32 => "monday",
    1_u32 => "tuesday",
    2_u32 => "wednesday",
    3_u32 => "thursday",
    4_u32 => "friday",
    5_u32 => "saturday",
    6_u32 => "sunday"
};

#[derive(Debug, Clone)]
struct Entry {
    title: String,
    id: usize,
    watched_episodes: usize,
    total_episodes: usize,
    weekday: Option<Weekday>,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text = format!("[{}] {}", self.watched_episodes.to_string().cyan(), self.title);
        if text.len() > MAX_ENTRY_TITLE_LENGTH {
            text.truncate(MAX_ENTRY_TITLE_LENGTH - 3);
            text.push_str("...");
        }
        write!(f, "{text}")
    }
}

#[derive(Copy, Clone)]
pub enum EpisodeAction {
    Set,
    Increment,
}

#[derive(Deserialize)]
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
    tags: Vec<String>,
}
#[derive(Deserialize)]
struct UserInfoResponse {
    name: String,
}

fn get_entries(auth: &AuthConfig) -> Result<Vec<Entry>> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut page: AnimeListResponse =
        ureq::get("https://api.myanimelist.net/v2/users/@me/animelist?status=watching&fields=my_list_status{tags,comments},num_episodes&nsfw=true")
            .set("Authorization", &format!("Bearer {}", auth.access_token))
            .call()?
            .into_json()?;

    loop {
        for data in &page.data {
            entries.push(Entry {
                title: data.node.title.clone(),
                id: data.node.id,
                watched_episodes: data.node.my_list_status.num_episodes_watched,
                total_episodes: data.node.num_episodes,
                weekday: WEEKDAY_MAPPINGS.get(&data.node.my_list_status.tags.first().unwrap_or(&String::new()).to_lowercase()).copied(),
            });
        }

        match page.paging.next {
            Some(url) => {
                page = ureq::get(&url).set("Authorization", &format!("Bearer {}", auth.access_token)).call()?.into_json()?;
            }
            None => break,
        }
    }

    entries.sort_by(|a, b| a.title.cmp(&b.title));

    Ok(entries)
}

fn select_entry(entries: &[Entry]) -> Result<Entry> {
    let formatter: OptionFormatter<Entry> = &|e| e.value.title.to_string();
    let entries_prompt = Select::new("Select an anime you are currently watching:", entries.to_vec()).with_formatter(formatter).with_page_size(20);

    Ok(entries_prompt.prompt()?)
}

fn base_update_entry_request(auth: &AuthConfig, entry: &Entry) -> ureq::Request {
    ureq::patch(&format!("https://api.myanimelist.net/v2/anime/{}/my_list_status", entry.id))
        .set("Authorization", &format!("Bearer {}", auth.access_token))
}

pub fn display_currently_watching_list(auth: &AuthConfig, sp: &mut spinners::Spinner) -> Result<()> {
    let entries = get_entries(auth)?;
    spinner::stop_spinner(sp)?;

    let mut seasonal_entry_vectors: Vec<Vec<Entry>> = vec![Vec::new(); 7];
    let mut off_season_entries: Vec<Entry> = Vec::new();

    let today = Local::now().weekday();

    for entry in entries {
        match entry.weekday {
            Some(weekday) => {
                let index = (7 + weekday.num_days_from_monday() - today.num_days_from_monday()) % 7;
                seasonal_entry_vectors[index as usize].push(entry);
            }
            None => off_season_entries.push(entry),
        }
    }

    if !off_season_entries.is_empty() {
        println!("{}:", "Off-season".magenta().underline());
        for off_season_entry in off_season_entries {
            println!("  {off_season_entry}");
        }
    }

    for vector in seasonal_entry_vectors {
        if let Some(entry) = vector.first() {
            let Some(weekday) = entry.weekday else {
                unreachable!();
            };
            if weekday == today {
                println!("{}:", weekday.to_string().green().underline());
            } else {
                println!("{}:", weekday.to_string().magenta().underline());
            }
            for seasonal_entry in vector {
                println!("  {seasonal_entry}");
            }
        }
    }

    Ok(())
}

pub fn update_episode_count(auth: &AuthConfig, sp: &mut spinners::Spinner, action: EpisodeAction) -> Result<()> {
    let entries = get_entries(auth)?;
    spinner::stop_spinner(sp)?;

    loop {
        let entry = select_entry(&entries)?;

        let confirm_prompt_text = format!("Update \"{}\"", entry.title);
        let help_message_text = format!(
            "{} -> {}/{} episodes",
            entry.watched_episodes,
            match action {
                EpisodeAction::Set => "N".to_string(),
                EpisodeAction::Increment => (entry.watched_episodes + 1).to_string(),
            },
            if entry.total_episodes == 0 {
                "?".to_string()
            } else {
                entry.total_episodes.to_string()
            }
        );

        if Confirm::new(&confirm_prompt_text).with_default(true).with_help_message(&help_message_text).prompt()? {
            let request = base_update_entry_request(auth, &entry);

            let new_episode_count: usize = match action {
                EpisodeAction::Set => CustomType::new("Input episode count:").with_error_message("Invalid episode count!").prompt()?,
                EpisodeAction::Increment => entry.watched_episodes + 1,
            };

            let mut set_completed = false;
            if new_episode_count == entry.total_episodes && entry.total_episodes != 0 {
                set_completed = Confirm::new(&format!("Set \"{}\" as completed?", entry.title)).with_default(true).prompt()?;
            }

            if set_completed {
                let score = CustomType::new("Input score (0-10):")
                    .with_parser({
                        &|n| {
                            let num = n.parse::<usize>();
                            match num {
                                Ok(0..=10) => Ok(num.unwrap_or(10)),
                                _ => Err(()),
                            }
                        }
                    })
                    .with_error_message("Invalid score!")
                    .prompt()?;
                let review = Text::new("Input review:").prompt()?;

                request.send_form(&[
                    ("num_watched_episodes", new_episode_count.to_string().as_str()),
                    ("status", "completed"),
                    ("score", score.to_string().as_str()),
                    ("tags", &review),
                ])?;
            } else {
                request.send_form(&[("num_watched_episodes", new_episode_count.to_string().as_str())])?;
            }

            break;
        }
    }

    Ok(())
}

pub fn update_airing_day(auth: &AuthConfig, sp: &mut spinners::Spinner) -> Result<()> {
    let entries = get_entries(auth)?;
    spinner::stop_spinner(sp)?;

    let entry = select_entry(&entries)?;

    let request = base_update_entry_request(auth, &entry);

    let mut days = VecDeque::from([
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ]);
    for _ in 0..(Local::now().weekday().num_days_from_monday()) {
        let day = days.pop_front().expect("days VecDeque shouldn't be empty");
        days.push_back(day);
    }

    let prompt_text = format!("Select an airing day to set for \"{}\"", &entry.title);
    let weekday = Select::new(&prompt_text, days.into()).prompt()?;
    let airing_day = AIRING_DAY_MAPPINGS.get(&weekday.num_days_from_monday()).copied().unwrap_or_default();

    request.send_form(&[("tags", airing_day)])?;

    Ok(())
}

pub fn open_my_anime_list(auth: &AuthConfig, sp: &mut spinners::Spinner) -> Result<()> {
    spinner::stop_spinner(sp)?;

    let response: UserInfoResponse =
        ureq::get("https://api.myanimelist.net/v2/users/@me").set("Authorization", &format!("Bearer {}", auth.access_token)).call()?.into_json()?;

    open::that(format!("https://myanimelist.net/animelist/{}?status=1", response.name))?;

    Ok(())
}

pub fn open_anime_page(auth: &AuthConfig, sp: &mut spinners::Spinner) -> Result<()> {
    let entries = get_entries(auth)?;
    spinner::stop_spinner(sp)?;

    let entry = select_entry(&entries)?;

    open::that(format!("https://myanimelist.net/anime/{}", entry.id))?;

    Ok(())
}
