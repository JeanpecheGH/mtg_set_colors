use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::ops::Add;
use std::str::FromStr;

mod http;
use http::Card;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Rarity {
    M,
    R,
    U,
    C,
}

impl FromStr for Rarity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" | "m" => Ok(Rarity::M),
            "R" | "r" => Ok(Rarity::R),
            "U" | "u" => Ok(Rarity::U),
            "C" | "c" => Ok(Rarity::C),
            _ => Err(String::from(
                "Invalid rarity. Chose one or more among M,R,U,C",
            )),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser = parse_set)]
    /// The target MTG Set Trigram
    set: String,
    #[clap(short, long, multiple_values = true, max_values = 4, default_values = &["M","R","U"])]
    /// Space separated target rarities among M,R,U,C
    rarity: Vec<Rarity>,
}

fn main() {
    let args = Args::parse();
    let rarities: HashSet<Rarity> = args.rarity.iter().cloned().collect();
    //Get data from scryfall, parse and write to file
    rarities.iter().for_each(|r| {
        let data = to_csv(http::get_cards(&args.set, r).unwrap());
        let filename = format!("{}.{:#?}.csv", &args.set, r);
        fs::write(filename, data).expect("Unable to write file");
    })
}

fn parse_set(set: &str) -> Result<String, String> {
    if set.trim().len() != set.len() {
        Err(String::from("Set cannot have leading and trailing space"))
    } else if set.len() != 3 {
        Err(String::from("Set must be a trigram"))
    } else {
        Ok(String::from(set))
    }
}

fn to_csv(cards: Vec<Card>) -> String {
    let v: Vec<String> = cards.iter().map(|c| c.to_line()).collect();
    v.join("\n").add("\n")
}
