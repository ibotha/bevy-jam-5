use rand::Rng;
use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn join_card_game(actions: &mut StoryActions) {
    let luck = actions.get_rng().gen_range(-50..=100);
    if luck > 0 {
        actions.delta_items(Item::Gold, luck);
        actions.add_dialogue(captain!(format!("Lady Luck smiled on us tonight! We won {} gold in the card game.", luck)));
    } else {
        actions.delta_items(Item::Gold, luck);
        actions.add_dialogue(captain!(format!("The cards weren't in our favor. We lost {} gold, but at least the crew had some fun.", -luck)));
    }
}

fn buy_round_of_drinks(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 30 {
        actions.delta_items(Item::Gold, -30);
        actions.delta_crew(1);
        actions.delta_health(5);
        actions.add_dialogue(captain!("The crew's morale is high after a round of drinks. We even convinced a local sailor to join our crew, and someone shared tips on ship maintenance."));
    } else {
        actions.delta_items(Item::Gold, -10);
        actions.delta_food(-5);
        actions.add_dialogue(captain!("We could only afford a small round, but it lifted spirits. We shared some of our provisions with friendly locals."));
    }
}

fn listen_to_local_music(actions: &mut StoryActions) {
    actions.delta_food(-3);
    actions.delta_health(3);
    actions.add_dialogue(captain!("We enjoyed the local tunes and relaxed. It was a peaceful evening that did wonders for the crew's morale."));
}

pub fn the_local_tavern_visit_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, the crew's eager for some shore leave. The local tavern's quite popular among sailors."))
        .line(captain!("Ah, a bit of relaxation might do us all some good. What's on offer at this establishment?"))
        .line(crew2!("There's a lively card game going on in the corner, Cap'n. Could be a chance to win some gold."))
        .line(crew3!("The drinks are flowing, and there's talk of some fine local music later tonight."))
        .line(captain!("Sounds like a typical tavern night. What shall we do?"))
        .choice("Join", join_card_game)
        .conditional_choice("Buy drinks", buy_round_of_drinks, actions.get_item(Item::Gold) >= 10)
        .choice("Enjoy time", listen_to_local_music)
        .hint("Squawk! A merry crew makes for a swift journey, but a light purse makes for a heavy heart!")
}