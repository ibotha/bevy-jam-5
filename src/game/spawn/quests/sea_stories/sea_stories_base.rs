use crate::game::spawn::quests::prelude::*;

pub fn sail(actions: &mut StoryActions) {
    actions.travel(actions.possible_distance().min(actions.get_crew()));
    actions.delta_crew(-actions.danger() / 3);
}

fn rest(actions: &mut StoryActions) {
    actions.delta_crew(1);
}

fn hunker_down(_actions: &mut StoryActions) {}

pub fn sea_stories_base(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .choice("Sail", sail)
        .choice("Hunker", hunker_down)
}