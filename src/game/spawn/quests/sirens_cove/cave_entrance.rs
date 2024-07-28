use crate::game::spawn::quests::prelude::*;

use super::final_encounter::final_encounter;

fn queue_final_encounter(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: final_encounter,
        delay: Delay::None,
        certainty: Certainty::Certain,
        environment: actions.get_environment(),
    });
}

fn pool(actions: &mut StoryActions) {
    actions.add_dialogue(crew2!("Into the water we go."));
    let gf = actions.get_item(Item::GreekFire) > 0;
    match (gf, actions.weather().heat) {
        (false, H::Freezing | H::Chilly) => {
            actions.add_dialogue(captain!("Damn this water is freezing."));
            actions.add_dialogue(narrator!("Once you come out the other end you crew begin to freeze, those with existing wounds are not able to fight it off."));
            actions.add_dialogue(captain!("Just a little longer to go. I can see it now."));
            actions.delta_crew(-2);
        }
        _ => {
            actions.add_dialogue(captain!("Just a little water, nothing to be afraid of"));
            actions.add_dialogue(captain!("Just a little longer to go. I can see it now."));
        }
    }
    queue_final_encounter(actions);
}

fn tunnels(actions: &mut StoryActions) {
    actions.add_dialogue(crew2!("The tunnels are more suited to folks with legs."));
    if actions.danger() > 3 {
        actions.add_dialogue(crew3!("Flash flood. Hold on to something!"));
        actions.add_dialogue(narrator!("Your crew gets wisked away by a sudden current. Those of you that survive can see a glow from the next chamber."));
        actions.delta_crew(-actions.danger() / 2);
    } else {
        actions.add_dialogue(captain!("Just a little longer to go. I can see it now."));
    }
    queue_final_encounter(actions);
}

pub fn cave_entrance(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew3!("I think I found the entrance to the Siren's Lair!"))
        .line(crew1!(
            "What makes you say that? It just looks like a cave."
        ))
        .line(crew3!(
            "The paintings on the walls. They show sirens swimming through the pool in the cave."
        ))
        .line(captain!(
            "Smells of sirens in here.",
            "Looks like we got to get our swimming trunks on lads."
        ))
        .line(crew2!(
            "I see tunnels in the cave, they might lead us through safer.",
            "Not all the crew are strong swimmers."
        ))
        .hint("*squawk* RaiNS can FLOod. FlASH a FLOod!")
        .line(captain!("Which path do we brave?"))
        .choice("Pool", pool)
        .choice("Tunnels", tunnels)
}
