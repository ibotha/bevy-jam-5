use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn accept_challenge(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Dry) => {
            actions.delta_items(Item::Gold, 2000);
            actions.delta_items(Item::Cannon, 2);
            actions.delta_health(20);
            actions.add_dialogue(captain!("By all that's holy! We've bested the ghost captain in his challenges of skill and wit. The spectral crew honored their word, gifting us their ethereal treasure and two enchanted cannons. The experience has somehow strengthened our very ship!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("We completed the ghost captain's trials, but at a cost. Poor Jenkins was spirited away during the final challenge. Still, we've won a king's ransom in ghostly gold, even if the whole ordeal has left us a bit shaken."));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_crew(-3);
            actions.delta_health(-30);
            actions.add_dialogue(captain!("Curse that phantom and his blasted ship! The challenges turned deadly in this weather. We lost three good men to the ghost crew, and our ship's barely holding together. We're lucky to have escaped with our lives and souls intact!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_food(-25);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We managed to complete most of the ghost captain's challenges. He seemed impressed enough to award us some spectral gold, but the ordeal has left us drained and our supplies diminished."));
        }
    }
}

fn negotiate_passage(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 500 {
        actions.delta_items(Item::Gold, -500);
        actions.delta_items(Item::Cannon, 1);
        actions.delta_health(15);
        actions.add_dialogue(captain!("The ghost captain accepted our offer of gold, impressed by our respectful approach. In return, he's granted us safe passage and gifted us with a cannon of ghostly origin. The crew's spirits are high after this supernatural encounter!"));
    } else {
        actions.delta_health(-20);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("Without enough gold to offer, the ghost captain took offense. He cursed our ship before vanishing, taking one of our crew with him. The remaining crew is terrified, and the ship feels... wrong."));
    }
}

fn flee_ghost_ship(actions: &mut StoryActions) {
    actions.delta_food(-40);
    actions.delta_health(-15);
    actions.delta_items(Item::Gold, 100);
    actions.add_dialogue(captain!("We turned tail and fled from that cursed ship, pushing our vessel to its limits. The escape has drained our supplies and stressed the ship, but as we sailed away, we found a small chest of gold floating in our wake. A parting gift... or a cruel joke?"));
}

pub fn the_ghost_ships_challenge_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a ship approaching... but it's transparent as mist!"))
        .line(captain!("Shiver me timbers... a ghost ship! What in blazes does it want?"))
        .line(crew2!("They're hailing us, Cap'n! The ghost captain challenges us to a series of trials. Says if we win, their treasure is ours!"))
        .line(crew3!("We could try to bargain for safe passage instead, Cap'n. Might cost us some gold, but it'd be safer than taking on their challenges."))
        .line(captain!("This is unlike anything we've faced before, lads. Our next move could lead us to fortune or doom. What say you?"))
        .choice("Accept Challenge", accept_challenge)
        .conditional_choice("Negotiate", negotiate_passage, actions.get_item(Item::Gold) >= 500)
        .choice("Flee", flee_ghost_ship)
        .hint("Squawk! The dead tell no tales, but they sure do love their games!")
}