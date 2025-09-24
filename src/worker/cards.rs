use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardList {
    data: Vec<Card>,
}

impl CardList {
    pub fn to_lines(&self, printed_name: bool) -> Vec<String> {
        let mut lines: Vec<String> = self.data.iter().map(|c| c.to_line(printed_name)).collect();
        if printed_name {
            lines.sort();
        }
        lines
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    name: String,
    printed_name: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Option<Vec<String>>,
    card_faces: Option<Vec<CardFace>>,
}

impl Card {
    fn to_line(&self, printed_name: bool) -> String {
        let n: &str = match (printed_name, &self.printed_name, &self.card_faces) {
            (true, Some(p),_) => p,
            (true, None, Some(faces)) => &format!("{} // {}", faces[0].printed_name(), faces[1].printed_name()),
            _ => &self.name,
        };
        format!("{};{}", n, self.color())
    }

    fn color(&self) -> char {
        fn extract_color(id: &[String]) -> char {
            match id.len() {
                0 => 'C',
                1 => id[0].chars().next().unwrap(),
                _ => 'M',
            }
        }
        self.colors.as_ref().or(self.color_identity.as_ref()).map(|v| extract_color(v)).unwrap_or('C')
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardFace {
    name: String,
    printed_name: Option<String>,
}

impl CardFace {
    fn printed_name(&self) -> String {
        self.printed_name.as_ref().unwrap_or(&self.name).to_string()
    }
}