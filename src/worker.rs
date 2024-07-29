use crate::Rarity;
use serde_json::Value;
use std::ops::Add;
use std::sync::Arc;
use reqwest::header::{HeaderMap, USER_AGENT};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Card {
    name: String,
    color: char,
}

impl Card {
    fn get_color(colors: Vec<char>) -> char {
        match colors.len() {
            0 => 'C',
            1 => colors[0],
            _ => 'M',
        }
    }

    pub fn to_line(&self) -> String {
        format!("{};{}", self.name, self.color)
    }
}

pub async fn get_cards(set: Arc<String>, rarity: Rarity) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Retrieving cards info for set : {}, rarities {:#?}",
        set, rarity
    );
    let json = get_cards_data(set.as_str(), &rarity).await?;
    let cards = parse_data(json).await?;
    write_cards_to_file(set.as_str(), &rarity, cards).await?;
    Ok(())
}

async fn get_cards_data(set: &str, rarity: &Rarity) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.scryfall.com/cards/search?q=set%3A{}+r%3A{:?}+is%3Abooster",
        set, rarity
    );
    println!("Url : {:#?}", url);

    let client = reqwest::Client::new();

    // Add User agent to a HeaderMap
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "MtgSetColors/0.1.1".parse().unwrap());

    let resp = client.get(url).headers(headers).send().await?.json::<Value>().await?;
    Ok(resp)
}

async fn parse_data(v: Value) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let cards: Vec<Card> = v["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let name: String = v["name"].to_string().replace('"', "");
            let field: &str = if v["colors"].is_null() {
                "color_identity"
            } else {
                "colors"
            };
            let colors: Vec<char> = v[field]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c.to_string().replace('"', "").chars().next().unwrap())
                .collect();
            let color: char = Card::get_color(colors);
            Card { name, color }
        })
        .collect();
    Ok(cards)
}

async fn write_cards_to_file(
    set: &str,
    rarity: &Rarity,
    cards: Vec<Card>,
) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<String> = cards.iter().map(|c| c.to_line()).collect();
    let data = v.join("\n").add("\n");
    let filename = format!("{}.{:#?}.csv", set, rarity);
    let mut file = File::create(filename).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}
