use super::prelude::*;

pub fn to_sirens_cove(actions: &mut StoryActions) {
    todo!()
}

pub fn edge_of_the_world(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!(
            "We stand at the precipice of our greatest prize!",
            "Is now the time to enter siren's cove?"
        ))
        .choice("Yes", to_sirens_cove)
}
