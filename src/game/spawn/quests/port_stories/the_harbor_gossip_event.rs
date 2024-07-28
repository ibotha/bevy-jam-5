use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;
use rand::Rng;

fn listen_to_rumors(actions: &mut StoryActions) {
    let rumor_type = actions.get_rng().gen_range(0..=3);
    match rumor_type {
        0 => {
            actions.add_dialogue(captain!("Heard whispers of a wealthy merchant ship setting sail soon. Could be a good target for... trade negotiations."));
            // We can potentially add a future event
        }
        1 => {
            actions.delta_health(5);
            actions.add_dialogue(captain!("Got wind of a shortcut through some treacherous waters. Might save us some wear and tear on future voyages."));
        }
        2 => {
            actions.delta_food(-5);
            actions.delta_items(Item::Gold, 30);
            actions.add_dialogue(captain!("Learned of a merchant looking for rare spices. Sold him some of our stock at a premium."));
        }
        _ => {
            actions.add_dialogue(captain!("Nothing too interesting in the rumor mill today. At least we know what not to waste our time on."));
        }
    }
}

fn share_tales(actions: &mut StoryActions) {
    if actions.get_crew() > 5 {
        actions.delta_crew(1);
        actions.delta_food(-5);
        actions.add_dialogue(captain!("Our tales of adventure impressed the locals. We gained a new crew member, though we had to buy a round of drinks for everyone."));
    } else {
        actions.delta_items(Item::Gold, 20);
        actions.add_dialogue(captain!("Our stories entertained the crowd. A few listeners tossed some coins our way in appreciation."));
    }
}

fn avoid_gossip(actions: &mut StoryActions) {
    actions.delta_health(3);
    actions.add_dialogue(captain!("We kept to ourselves and focused on ship maintenance. It's not exciting, but it keeps us out of trouble."));
}

pub fn the_harbor_gossip_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, the harbor's abuzz with chatter today. Lots of interesting tidbits floating around."))
        .line(captain!("Is that so? And what sort of gossip has caught your ear?"))
        .line(crew2!("All sorts, Cap'n. Talk of trade routes, pirate sightings, and hidden treasures."))
        .line(crew3!("Some folks are eager to hear our own tales of the sea as well."))
        .line(captain!("Interesting. How shall we approach this wealth of information?"))
        .choice("Listen", listen_to_rumors)
        .conditional_choice("Share tales", share_tales, actions.get_crew() > 3)
        .choice("Avoid", avoid_gossip)
        .hint("Squawk! A wise captain knows that knowledge is power, but lose lips sink ships!")
}
