use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn accept_bargain(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::None | W::Low, M::Dry | M::Comfortable) => {
            actions.delta_crew(-1);
            actions.delta_items(Item::Gold, 100);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("The Kraken kept its word! It took one crew member but left us with treasure and cannons from sunken ships. A steep price, but the reward is substantial."));
        }
        (W::Medium, _) => {
            actions.delta_crew(-1);
            actions.delta_health(20);
            actions.delta_food(20);
            actions.add_dialogue(captain!("The Kraken took our crew member and in return, patched up our ship with some sort of mystical kelp. We also found our food stores mysteriously replenished!"));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-1);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("The Kraken took our crew member, but the stormy seas made it difficult for it to provide compensation. It left us with only minor repairs and an apology!"));
        }
        _ => {
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("The Kraken took our crew member and left."));
        }
    }
}

fn attempt_negotiation(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_items(Item::Gold, 150);
        actions.delta_health(15);
        actions.delta_food(20);
        actions.add_dialogue(captain!("The Monkey's Paw glowed during our negotiation. The Kraken, fascinated by the artifact, agreed to our terms without taking a crew member! It even threw in some extra treasures and supplies."));
    } else {
        actions.delta_crew(-2);
        actions.delta_items(Item::Gold, 300);
        actions.add_dialogue(captain!("Our attempt to negotiate backfired. The Kraken demanded two crew members instead, but doubled the treasure in return. A costly bargain indeed."));
    }
}

fn refuse_and_flee(actions: &mut StoryActions) {
    actions.delta_health(-20);
    actions.delta_food(-15);
    actions.add_dialogue(captain!("We turned tail and fled as fast as we could. The Kraken didn't pursue, but the strain of our escape took a toll on the ship and our supplies."));
}

pub fn the_kraken_s_bargain_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("C-C-Cap'n! The Kraken! It's surfacing right next to us!"))
        .line(captain!("Steady, crew! Ready the cannons and... wait, what's this?"))
        .line(crew2!("It's... it's not attacking, sir. It seems to be communicating somehow."))
        .line(crew3!("Cap'n, I think it's offering us a deal. One crew member in exchange for treasure from sunken ships!"))
        .line(captain!("By Neptune's beard! A bargain with the Kraken itself? What's our move, crew?"))
        .choice("Accept Bargain", accept_bargain)
        .conditional_choice("Negotiate", attempt_negotiation, actions.get_item(Item::MonkeyPaw) > 0)
        //.choice("Refuse and Flee", refuse_and_flee)
        .hint("Squawk! Even the mightiest beasts of the sea can surprise you!")
}

