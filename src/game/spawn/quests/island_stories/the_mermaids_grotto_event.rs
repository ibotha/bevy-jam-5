use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn approach_mermaids(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Comfortable) => {
            actions.delta_items(Item::Gold, 2500);
            actions.delta_health(40);
            actions.delta_crew(2);
            actions.add_dialogue(captain!("Extraordinary! The mermaids were friendly and gifted us with ancient treasures from the depths. They've blessed our ship with magic that mends its wounds, and two of our injured crew have miraculously recovered!"));
        }
        (W::Medium, M::Humid) => {
            actions.delta_items(Item::Gold, 1200);
            actions.delta_crew(-1);
            actions.delta_health(20);
            actions.add_dialogue(captain!("The mermaids shared their treasures, but their magic is unpredictable. We've gained gold and our ship feels rejuvenated, but one of our crew transformed into a merman and swam away!"));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_crew(-3);
            actions.delta_health(-25);
            actions.add_dialogue(captain!("Neptune's wrath! The mermaids turned hostile in this turbulent weather. Their siren song lured three of our crew to a watery grave, and their magic has damaged our ship. We're lucky to escape with our lives!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 800);
            actions.delta_food(40);
            actions.delta_health(10);
            actions.add_dialogue(captain!("The mermaids were cautious but not unfriendly. They've gifted us some gold and magical food that will keep us healthy for months. The ship feels a bit stronger too, though the encounter has left us all a bit dazed."));
        }
    }
}

fn offer_trade(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 1 {
        actions.delta_items(Item::Cannon, -1);
        actions.delta_items(Item::Gold, 3000);
        actions.delta_health(30);
        actions.add_dialogue(captain!("The mermaids were fascinated by our cannon! In exchange, they've given us a fortune in gold and pearls, and bestowed a blessing that's mended our ship and crew. Who knew mermaids had a taste for human technology?"));
    } else {
        actions.delta_items(Item::Gold, -200);
        actions.delta_food(20);
        actions.add_dialogue(captain!("Without anything unique to offer, we traded some of our gold for magical supplies. The mermaids seemed a bit disappointed, but the enchanted food they provided will keep us healthy for a while."));
    }
}

fn observe_from_distance(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 300);
    actions.delta_health(5);
    actions.delta_food(10);
    actions.add_dialogue(captain!("We kept our distance and observed the mermaids from afar. They noticed us and, seemingly amused by our caution, left some gifts on a nearby rock. We found gold, some magical fruit, and the experience has oddly improved our navigational skills."));
}

pub fn the_mermaids_grotto_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a hidden grotto ahead... and I swear I saw mermaids!"))
        .line(captain!("Mermaids? Are ye sure yer eyes aren't playing tricks on ye?"))
        .line(crew2!("It's true, Cap'n! Beautiful creatures, half-fish and half-human. They seem to be guarding something shiny."))
        .line(crew3!("I've heard tales of mermaid magic, Cap'n. Could be dangerous... or incredibly rewarding if we approach carefully."))
        .line(captain!("This could change our fortunes forever, crew. But we must tread carefully. What shall we do?"))
        .choice("Approach Mermaids", approach_mermaids)
        .conditional_choice("Offer Trade", offer_trade, actions.get_item(Item::Cannon) >= 1)
        .choice("Observe Distance", observe_from_distance)
        .hint("Squawk! Beauty and danger often swim in the same waters!")
}