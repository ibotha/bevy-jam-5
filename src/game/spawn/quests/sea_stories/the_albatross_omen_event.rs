use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn follow_albatross(actions: &mut StoryActions) {
    let DW {
        heat,
        wind: _,
        moisture: _,
    } = actions.weather();

    match heat {
        H::Comfortable | H::Warm => {
            actions.delta_items(Item::Gold, 10);
            actions.delta_food(15);
            actions.add_dialogue(captain!("The albatross led us to calm waters teeming with fish, and we stumbled upon a small treasure on a nearby islet!"));
        }
        H::Chilly | H::Freezing => {
            actions.delta_health(-10);
            actions.delta_food(25);
            actions.add_dialogue(captain!("The bird led us into colder waters. We took some damage from ice, but found an abundance of fish."));
        }
        H::Blistering => {
            actions.delta_crew(-1);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("The albatross led us into a terrible storm! We lost a crew member and some of our supplies got spoiled."));
        }
    }
}

fn capture_albatross(actions: &mut StoryActions) {
    actions.delta_food(10);
    actions.delta_items(Item::Gold, -25);
    actions.add_dialogue(captain!("We managed to capture the albatross. It'll make for a good meal, but the crew's morale has taken a hit. They say it's bad luck to kill an albatross."));
    // You might want to add a negative effect later due to killing the albatross
}

fn ignore_albatross(actions: &mut StoryActions) {
    actions.delta_food(5);
    actions.add_dialogue(captain!("We kept to our course, ignoring the albatross. Nothing unusual happened, but we had a decent day of fishing."));
}

pub fn the_albatross_omen_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's an albatross circling our ship!"))
        .line(captain!("An albatross, you say? That's often considered a sign of good fortune."))
        .line(crew2!("Aye, sir. But some say it can be an omen too, depending on how we treat it."))
        .line(crew3!("What should we do, Cap'n? Follow it? Try to catch it? Or just ignore it?"))
        .line(captain!("Hmm, this could be interesting. What's your gut telling you, crew?"))
        .choice("Follow", follow_albatross)
        .choice("Capture", capture_albatross)
        .choice("Ignore", ignore_albatross)
        .hint("Squawk! The winds of fate often blow in mysterious directions!")
}