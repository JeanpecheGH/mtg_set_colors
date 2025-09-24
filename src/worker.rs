use crate::{Args, Rarity};
use std::ops::Add;
use std::sync::Arc;
use clap::{crate_name, crate_version};
use reqwest::header::{HeaderMap, USER_AGENT};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::worker::cards::CardList;

mod cards;

pub async fn get_cards(args: Arc<Args>, rarity: Rarity) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Retrieving cards info for set : {}, rarities {:#?}, use all cards {}, use printed name {}",
        args.set, rarity, args.all_cards, args.printed_name
    );
    let card_list: CardList = get_cards_data(&args, &rarity).await?;
    write_cards_to_file(&args, &rarity, card_list).await?;
    Ok(())
}

async fn get_cards_data(args: &Arc<Args>, rarity: &Rarity) -> Result<CardList, Box<dyn std::error::Error>> {
    let all_cards_param: &str = if args.all_cards {
        ""
    } else {
        "+is%3Abooster"
    };
    let url = format!(
        "https://api.scryfall.com/cards/search?q=set%3A{}+r%3A{:?}{}",
        args.set, rarity, all_cards_param
    );
    println!("Url : {:#?}", url);

    let client = reqwest::Client::new();

    // Add User agent to a HeaderMap
    let mut headers = HeaderMap::new();
    let agent: String = format!("{}/{}", crate_name!(), crate_version!());
    headers.insert(USER_AGENT, agent.parse().unwrap());

    let card_list: CardList  = client.get(url).headers(headers).send().await?.json::<CardList>().await?;
    Ok(card_list)
}

async fn write_cards_to_file(
    args: &Arc<Args>,
    rarity: &Rarity,
    card_list: CardList,
) -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<String> = card_list.to_lines(args.printed_name);
    let data = v.join("\n").add("\n");
    let filename = format!("{}.{:#?}.csv", args.set, rarity);
    let mut file = File::create(filename).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}
