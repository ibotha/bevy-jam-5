use crate::game::spawn::quests::prelude::*;
use crate::game::spawn::quests::sea_events::sail;

fn embark(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea);
    sail(actions);
}

pub fn port_stories_base(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new().choice("Embark", embark)
}

