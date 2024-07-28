use crate::game::spawn::quests::{northern_seas::set_course_northern_sea, prelude::*};

pub fn sail(actions: &mut StoryActions) {
    actions.travel(actions.possible_distance().min(actions.get_crew().max(4)));
    if actions.danger() > 5 {
        actions.delta_crew(-actions.danger() / 3);
    }
}

fn hunker_down(actions: &mut StoryActions) {
    actions.travel((actions.possible_distance() / 2).min(actions.get_crew()));
}

pub fn sea_stories_base(actions: &mut StoryActions) -> DayEvent {
    let e = if actions.get_current_sea() == Sea::Northern && actions.no_course_set() {
        set_course_northern_sea(actions)
            .line(crew1!("We have no heading captain, should we choose one?"))
    } else {
        DayEvent::new()
    };
    e.choice("Sail", sail).choice("Hunker", hunker_down)
}

