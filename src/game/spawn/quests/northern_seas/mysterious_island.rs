use super::{super::prelude::*, set_course_northern_sea};

fn enter_mysterious_island(actions: &mut StoryActions) {
    actions.change_environment(Environment::Island(Island::MysteriousIsland));
    actions.add_event(FollowingEvent {
        environment: Environment::Island(Island::MysteriousIsland),
        certainty: Certainty::Certain,
        delay: Delay::Days(3),
        event: mysterious_encounter,
    });
}

fn weigh_anchor(actions: &mut StoryActions) {
    if actions.danger() > 5 {
        actions.add_dialogue(captain!(
            "With how this weather is turning out, we made the right call."
        ));
    } else {
        actions.add_dialogue(captain!(
            "We could have brought more men. Well we have laid our bed."
        ));
    }
    actions.island_crew(3);
    enter_mysterious_island(actions);
}

fn risk_the_shallows(actions: &mut StoryActions) {
    if actions.danger() > 5 {
        actions.add_dialogue(crew1!(
            "The ship is out of control captain! Everyone brace!"
        ));
        actions.delta_health(-actions.danger());
    } else {
        actions.add_dialogue(captain!(
            "Looks like we made it nice and close. Lets go investigate!"
        ));
    }
    enter_mysterious_island(actions);
}

pub fn sighted_mysterious_island(actions: &mut StoryActions) -> DayEvent {
    set_course_northern_sea(actions)
        .line(crew3!("There it is captain!", "The mysterious island."))
        .line(captain!("What do you think sage?", "Do we take another heading, send a small party, or risk the ship navigating the shallows?"))
        .choice("Weigh Anchor", weigh_anchor)
        .choice("Shallows", risk_the_shallows)
}

fn accept_gift(actions: &mut StoryActions) {
    actions.add_dialogue(widow!("May it help you on your journey."));
    actions.delta_items(Item::SirensScale, 3);
}
fn reject_gift(actions: &mut StoryActions) {
    actions.add_dialogue(widow!(
        "Oh... very well then. I think I would like to be left alone now."
    ))
}
pub fn mysterious_encounter(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("Is that... a house?"))
        .line(crew2!("Who in their right mind would live all the way out here?"))
        .line(captain!("Someone that has been through a great deal I imagine..."))
        .line(captain!("*Knocks on door* Anyone home?"))
        .line(widow!("Go away!"))
        .line(captain!("Ma'am, I just want to sit down and have a come of tea for a while, how does that sound?"))
        .line(widow!("T-Tea? Yes... that sounds nice."))
        .line(narrator!("The door opens and a old, small, gaunt lady stands in the doorway."))
        .line(captain!("Well aren't you a lovely sight. How have you been?"))
        .line(widow!("Oh, here and there. But I must go on."))
        .line(captain!("Did you loose something."))
        .line(widow!("A long time ago. To creatures that you probably don't even believe in. But I have proof."))
        .line(narrator!("She idly runs her fingers over her necklace."))
        .line(widow!("You know, I can harldy remember the last time someone showed me kindniss."))
        .line(widow!("Take this, it only serves as a reminder of dark times these days."))
        .choice("Accept", accept_gift)
        .choice("Reject", reject_gift)
}
