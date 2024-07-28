use crate::game::spawn::quests::prelude::*;
use crate::game::spawn::quests::sea_stories::sail;

use super::cave_entrance::cave_entrance;

fn queue_cave_entrance(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: cave_entrance,
        delay: Delay::Days(2),
        certainty: Certainty::Certain,
        environment: actions.get_environment(),
    });
}

fn now(actions: &mut StoryActions) {
    actions.add_dialogue(crew1!("Alright, easy does it..."));
    if actions.get_clarity() > 5 {
        actions.add_dialogue(crew1!("Blast, it's spotted me..."));
        actions.add_dialogue(narrator!("The siren child dashes off into the woods..."));
    } else {
        actions.add_dialogue(crew1!("Haha! I got you!"));
        actions.delta_items(Item::SirenChild, 1);
        actions.add_dialogue(captain!("We have our leverage, let's get going."));
    }
    queue_cave_entrance(actions);
}

fn wait(actions: &mut StoryActions) {
    actions.add_dialogue(crew2!("Eyes like a hawk I have..."));
    actions.add_dialogue(narrator!(
        "Some time passes, the child simply waits and stares at the grave."
    ));
    if actions.get_clarity() > 5 {
        actions.add_dialogue(crew1!(
            "Its heading out, let us wait in ambush at the edge of the woods."
        ));
        actions.add_dialogue(crew1!("Haha! I got you!"));
        actions.delta_items(Item::SirenChild, 1);
        actions.add_dialogue(captain!("We have our leverage, let's get going."));
    } else {
        actions.add_dialogue(crew3!("This blasted fog, I can't make out the siren"));
        actions.add_dialogue(captain!("Damn, we lost it."));
    }
    queue_cave_entrance(actions);
}

fn leave_them_be(actions: &mut StoryActions) {
    actions.add_dialogue(captain!("Aye, thats too low, even for me."));
    queue_cave_entrance(actions);
}

pub fn burial_site(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!(
            "I think I see shaped stone ahead. I think it's a burial site."
        ))
        .line(crew3!("Burial site for what?"))
        .line(crew2!("Siren's Cove...", "Must be sirens."))
        .line(crew1!("Would it be best to investigate?"))
        .line(captain!("Aye. We'll investigate. You go ahead Patchy."))
        .line(crew1!("Aye Cap'n. Will be back in a spell"))
        .line(crew1!(
            "There's some creature there near the stone, alone.",
            "Looks small."
        ))
        .line(captain!("A siren child?"))
        .line(crew1!("Aye that would fit the bill Cap'n."))
        .line(crew3!("That could be our bargaining chip."))
        .c_line(
            crew2!(
                "Wait, a siren child, near a large stone...Thats King Triton's child!",
                "It says so in the prisoners journal"
            ),
            actions.get_item(Item::Journal) > 0,
        )
        .line(crew2!(
            "If the weather will provide cover we could sneak upon it?"
        ))
        .line(crew1!(
            "Or we could try wait for it to head out of the clearing."
        ))
        .choice("Let Them Be", leave_them_be)
        .choice("Sneak", now)
        .choice("Wait", wait)
}
