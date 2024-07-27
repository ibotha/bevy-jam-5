use super::{super::prelude::*, set_course_northern_sea};

fn weigh_anchor(actions: &mut StoryActions) {
    actions.change_environment(Environment::Island(Island::TrinketSeller));
    todo!();
}

fn risk_the_shallows(actions: &mut StoryActions) {
    actions.change_environment(Environment::Island(Island::TrinketSeller));
    todo!();
}

pub fn sighted_trinket_seller(actions: &mut StoryActions) -> DayEvent {
    set_course_northern_sea(actions)
        .line(crew3!("There it is captain!", "The mysterious island."))
        .line(captain!("What do you think sage?", "Do we take another heading, send a small party, or risk the ship navigating the shallows?"))
        .choice("Weigh Anchor", weigh_anchor)
        .choice("Shallows", risk_the_shallows)
}
