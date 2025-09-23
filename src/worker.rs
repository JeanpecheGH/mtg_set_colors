use crate::{Args, Rarity};
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

pub async fn get_cards(args: Arc<Args>, rarity: Rarity) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Retrieving cards info for set : {}, rarities {:#?}, use all cards {}, use printed name {}",
        args.set, rarity, args.all_cards, args.printed_name
    );
    let json = get_cards_data(args.set.as_str(), &rarity, args.all_cards).await?;
    let cards = parse_data(json, args.printed_name).await?;
    write_cards_to_file(args.set.as_str(), &rarity, cards).await?;
    Ok(())
}

async fn get_cards_data(set: &str, rarity: &Rarity, all_cards: bool) -> Result<Value, Box<dyn std::error::Error>> {
    let all_cards_param: &str = if all_cards {
        ""
    } else {
        "+is%3Abooster"
    };
    let url = format!(
        "https://api.scryfall.com/cards/search?q=set%3A{}+r%3A{:?}{}",
        set, rarity, all_cards_param
    );
    println!("Url : {:#?}", url);

    let client = reqwest::Client::new();

    // Add User agent to a HeaderMap
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "MtgSetColors/0.1.1".parse().unwrap());

    let resp = client.get(url).headers(headers).send().await?.json::<Value>().await?;
    Ok(resp)
}

async fn parse_data(v: Value, printed_name: bool) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let cards: Vec<Card> = v["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let name: String = if printed_name {
                if v["printed_name"].is_null() {
                    //If there is not printed name, we either have a double-face card or a card with a single name
                    if v["card_faces"].is_null() {
                        //Take the single name
                        v["name"].to_string().replace('"', "")
                    } else {
                        //We have to concatenate the names of the two sides
                        let faces_names: Vec<String> = v["card_faces"].as_array().unwrap().iter().map(|v| {
                            v["printed_name"].to_string().replace('"', "")
                        }).collect();
                        format!("{} // {}", faces_names[0], faces_names[1])
                    }
                } else {
                    v["printed_name"].to_string().replace('"', "")
                }

            } else {
                v["name"].to_string().replace('"', "")
            };
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
