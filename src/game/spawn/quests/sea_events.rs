use rand::RngCore;

use crate::game::weighted_random;

use super::{constants::CAPTAIN, day_event::DayEvent, dialogue::Dialogue, StoryActions};

fn sail(actions: &mut StoryActions) {
    actions.travel(10);
}

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(10);
}

fn woah(actions: &mut StoryActions) {
    actions.delta_crew(10);
    actions.delta_max_crew(10);
    actions.delta_health(10);
    actions.delta_max_health(10);
    actions.delta_food(10);
    actions.delta_max_food(10);
}

fn plain_sailing() -> DayEvent {
    DayEvent::new(
        &[Dialogue::new(
            CAPTAIN,
            &["Nothing on the horizon, should be a good day of smooth sailing."],
        )],
        &[("Sail", sail), ("Rest", rest), ("Woah", woah)],
    )
}

pub(super) fn select_random_sea_event(rng: &mut impl RngCore) -> DayEvent {
    weighted_random(Some(rng), &[(plain_sailing(), 14)]).clone()
}
