use super::prelude::*;

fn walk(actions: &mut StoryActions) {
    actions.travel(10);
}

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(10);
}

fn leave(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(actions.get_current_sea()));
}

fn just_walking(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!("Just more and more jungle."))
        .choice("Walk", walk)
        .choice("Rest", rest)
        .choice("Leave", leave)
}

pub(super) fn select_random_island_event(
    actions: &mut StoryActions,
    island: Island,
) -> EventBuilder {
    let choices = [(just_walking, 14)];
    weighted_random(Some(actions.get_journey_rng()), &choices).clone()
}
