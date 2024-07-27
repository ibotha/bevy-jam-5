use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn take_idol(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_health(10);
            actions.add_dialogue(captain!("We've secured the idol without incident! It's pure gold and heavier than it looks. The crew feels invigorated, as if blessed by this strange artifact."));
        }
        (H::Warm, W::Medium, _) => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_crew(-1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We got the idol, but at a cost. The temple started collapsing as we left. We lost a crewman and the ship took some damage in our hasty escape."));
        }
        (H::Blistering, _, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-2);
            actions.delta_health(-20);
            actions.add_dialogue(captain!("Cursed idol! As we tried to take it, the weather turned violent. We lost two men to mysterious accidents, and the ship's in bad shape. The idol vanished in the chaos!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_food(-20);
            actions.add_dialogue(captain!("We managed to take the idol, but the whole ordeal took longer than expected. It's valuable, but not as much as we hoped. Our supplies have dwindled in the process."));
        }
    }
}

fn study_idol(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -50);
    actions.delta_items(Item::Cannon, 1);
    actions.delta_food(30);
    actions.add_dialogue(captain!("Fascinating! The idol seems to be a key to ancient pirate caches. We've found a hidden store of cannons and provisions. The gold we spent on supplies for the study was well worth it."));
}

fn leave_idol(actions: &mut StoryActions) {
    actions.delta_health(15);
    actions.delta_crew(1);
    actions.add_dialogue(captain!("We decided not to meddle with forces we don't understand. As we left, we found a shipwrecked sailor who was drawn to the island. He's joined our crew, and everyone's relieved we avoided potential danger."));
}

pub fn the_mysterious_idol_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a strange golden idol in a clearing on this island!"))
        .line(captain!("An idol, you say? Tell me more."))
        .line(crew2!("It's perched on a stone pedestal, Cap'n. Looks valuable, but there's an eerie feeling about it."))
        .line(crew3!("I've heard tales of cursed idols bringing doom to greedy pirates. But if it's not cursed, it could fetch a hefty price."))
        .line(captain!("This could be a turning point for us, lads. What's our move?"))
        .choice("Take Idol", take_idol)
        .conditional_choice("Study Idol", study_idol, actions.get_item(Item::Gold) >= 50)
        .choice("Leave Idol", leave_idol)
        .hint("Squawk! Not all that glitters is gold, but sometimes it is!")
}