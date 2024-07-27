use rand::RngCore;

use super::prelude::*;

fn walk(actions: &mut StoryActions) {
    actions.travel(10);
}

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(10);
}

fn leave(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea);
}

fn just_walking() -> DayEvent {
    DayEvent::new()
        .line(captain!("Just more and more jungle."))
        .choice("Walk", walk)
        .choice("Rest", rest)
        .choice("Leave", leave)
}

pub(super) fn select_random_island_event(rng: &mut impl RngCore) -> DayEvent {
    weighted_random(Some(rng), &[(just_walking(), 14)]).clone()
}
