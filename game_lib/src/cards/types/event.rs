use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventCard {
    pub title: Title,
    pub description: Description,
    pub action: Effect,
}

impl EventCard {
    pub fn empty() -> Card {
        Card::Event(EventCard {
            title: Title::empty(),
            description: Description::empty(),
            action: Effect::NOP,
        })
    }
}
