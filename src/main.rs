use clap::Parser;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

mod worker;

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
    #[clap(short, long, value_delimiter = ',', default_values = &["M","R","U"])]
    /// Space separated target rarities among M,R,U,C
    rarity: Vec<Rarity>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let arc_set: Arc<String> = Arc::new(args.set);
    //Filter doubles
    let rarities: HashSet<Rarity> = args.rarity.iter().cloned().collect();

    //Get data from scryfall, parse and write to file
    let tasks: Vec<_> = rarities
        .into_iter()
        .map(|r| {
            let arc_set = arc_set.clone();
            tokio::spawn(async {
                let _ = worker::get_cards(arc_set, r).await;
            })
        })
        .collect();
    //Await all the tasks
    for task in tasks {
        task.await.unwrap();
    }
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
