use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn investigate_debris(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture: _,
    } = actions.weather();

    match wind {
        W::None | W::Low => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_food(10);
            actions.add_dialogue(captain!("Lucky day! We found some valuable cargo and preserved food supplies among the debris."));
        }
        W::Medium => {
            actions.delta_items(Item::Cannon, 1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We managed to salvage a cannon, but a shifting plank caused some minor damage to our hull."));
        }
        W::High | W::GaleForce => {
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("The rough seas made salvaging too dangerous! We lost a crew member and took some damage before we could retreat."));
        }
    }
}

fn salvage_wood(actions: &mut StoryActions) {
    actions.delta_health(15);
    actions.delta_food(-5);
    actions.add_dialogue(captain!("We gathered some good wood for repairs. The ship's in better shape now, but it took time away from fishing."));
}

fn avoid_debris(actions: &mut StoryActions) {
    actions.delta_food(5);
    actions.add_dialogue(captain!("We steered clear of the debris field. Took a bit longer, but we found some good fishing spots along the way."));
}

pub fn the_floating_debris_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's a large field of debris ahead!"))
        .line(captain!(
            "Debris, you say? Could be from a shipwreck or lost cargo. What can you see?"
        ))
        .line(crew2!(
            "Lots of floating wood, sir. Some crates too, but it's hard to tell what's in 'em."
        ))
        .line(crew3!(
            "Could be valuable salvage, Cap'n. But it might be risky to investigate too closely."
        ))
        .line(captain!(
            "Hmm, we have a decision to make. What shall we do?"
        ))
        .choice("Investigate", investigate_debris)
        .choice("Salvage Wood", salvage_wood)
        //.choice("Avoid", avoid_debris)
        .hint("Squawk! One ship's trash could be a pirate's treasure!")
}

