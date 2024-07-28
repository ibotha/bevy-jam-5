use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn participate_in_auction(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::None | W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, -20);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("Perfect conditions for a secret auction. We scored two cannons at a bargain price!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, -15);
            actions.delta_items(Item::MonkeyPaw, 1);
            actions.add_dialogue(captain!("We managed to acquire a mysterious monkey's paw. It's said to grant wishes, but at what cost?"));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_items(Item::Gold, -10);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("The strong winds drew attention to the auction. We lost gold and a crew member in the ensuing raid!"));
        }
        _ => {
            actions.delta_items(Item::Gold, -5);
            actions.delta_food(20);
            actions.add_dialogue(captain!("Nothing too exciting at the auction, but we got a good deal on some exotic provisions."));
        }
    }
}

fn inform_authorities(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 10);
    actions.delta_crew(-2);
    actions.add_dialogue(captain!("We did the 'right' thing and got a reward, but lost two crew members who thought we betrayed their trust."));
}

fn ignore_auction(actions: &mut StoryActions) {
    actions.delta_crew(1);
    actions.add_dialogue(captain!("We steered clear of the shady business. A fellow who respects our integrity decided to join our crew."));
}

pub fn the_smugglers_auction_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, I've caught wind of a secret auction happening in the port tonight."))
        .line(captain!("An auction? What's so secret about it?"))
        .line(crew2!("It's run by smugglers, Cap'n. They're selling all sorts of illegal and exotic goods."))
        .line(crew3!("Could be a chance to get our hands on some rare items, but it's risky business."))
        .line(captain!("Hmm, interesting. What are our options?"))
        .conditional_choice("Participate", participate_in_auction, actions.get_item(Item::Gold) >= 200)
        .choice("Inform authorities", inform_authorities)
        .choice("Ignore auction", ignore_auction)
        .hint("Squawk! In the world of smugglers, today's bargain could be tomorrow's bounty!")
}
