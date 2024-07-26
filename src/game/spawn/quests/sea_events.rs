use std::collections::VecDeque;

use rand::RngCore;

use crate::game::{
    spawn::{journey::Ship, weather::DayWeather},
    weighted_random,
};

use super::{constants::CAPTAIN, day_event::DayEvent, dialogue::Dialogue, ChoiceResult};

fn sail(mut ship: Ship, weather: &DayWeather) -> ChoiceResult {
    ship.distance_travelled = 10;
    ChoiceResult {
        ship,
        following_events: vec![],
        dialogues: VecDeque::default(),
    }
}

fn rest(mut ship: Ship, weather: &DayWeather) -> ChoiceResult {
    ship.crew += 10;
    ChoiceResult {
        ship,
        following_events: vec![],
        dialogues: VecDeque::default(),
    }
}

fn woah(mut ship: Ship, weather: &DayWeather) -> ChoiceResult {
    ship.crew += 10;
    ship.max_crew += 10;
    ship.health += 10;
    ship.max_health += 10;
    ship.distance_travelled = 10;
    ship.food += 10;
    ship.max_food += 10;
    ChoiceResult {
        ship,
        following_events: vec![],
        dialogues: VecDeque::default(),
    }
}

fn plain_sailing() -> DayEvent {
    DayEvent::new(
        &[Dialogue::new(
            CAPTAIN,
            &["Nothing on the horizon, should be a good day of smooth sailing."],
        )],
        &[("Sail", sail), ("Rest", rest), ("Woah", woah)],
    ).add_hints("Sleep now, cause you are tired!")
}

pub(super) fn select_random_sea_event(rng: &mut impl RngCore) -> DayEvent {
    weighted_random(Some(rng), &[(plain_sailing(), 14)])
}
