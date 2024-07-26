use rand::RngCore;

use crate::game::weighted_random;

use super::{constants::CAPTAIN, day_event::DayEvent, dialogue::Dialogue, StoryActions};

fn walk(actions: &mut StoryActions) {
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

fn just_walking() -> DayEvent {
    DayEvent::new(
        &[Dialogue::new(CAPTAIN, &["Just more and more jungle."])],
        &[("Walk", walk), ("Rest", rest), ("Woah", woah)],
    )
}

pub(super) fn select_random_island_event(rng: &mut impl RngCore) -> DayEvent {
    weighted_random(Some(rng), &[(just_walking(), 14)]).clone()
}
