
pub struct Card {
    message: String,
}

impl Card {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}