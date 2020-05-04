use crate::card::Card;

pub struct Player {
    pub is_tsar: bool,
    name: String,
    points: i32,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            is_tsar: false,
            points: 0,
            cards: vec![],
        }
    }

    pub fn card_count(&self) -> i32 {
        self.cards.len() as i32
    }

    pub fn award_point(&mut self) {
        self.points += 1;
    }

    pub fn deal_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}