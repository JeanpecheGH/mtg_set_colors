use crate::Rarity;

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

pub fn get_cards(set: &str, rarity: &Rarity) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.scryfall.com/cards/search?q=set%3A{}+r%3A{:?}+is%3Abooster",
        set, rarity
    );
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    let cards: Vec<Card> = resp["data"]
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
