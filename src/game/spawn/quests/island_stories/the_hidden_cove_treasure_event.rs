use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn explore_cave(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_health(5);
            actions.add_dialogue(captain!("Ahoy! We've struck gold in this cave! The conditions were perfect for exploration, and we've come out richer and stronger!"));
        }
        (H::Warm, W::Medium, M::Humid) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("We found some treasure, but the humid conditions made the cave treacherous. We lost one of our crew to a nasty fall."));
        }
        (H::Blistering, _, _) | (_, W::GaleForce, _) => {
            actions.delta_health(-10);
            actions.delta_food(-20);
            actions.add_dialogue(captain!("Blimey! The extreme conditions made the cave exploration a nightmare. We're lucky to have made it out alive, but we're worse for wear."));
        }
        _ => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("We found a small stash of gold, but the exploration took longer than expected. We've used up some of our provisions."));
        }
    }
}

fn negotiate_with_natives(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 200 {
        actions.delta_items(Item::Gold, -200);
        actions.delta_items(Item::Cannon, 1);
        actions.delta_food(30);
        actions.add_dialogue(captain!("The natives were willing to trade! We've acquired a new cannon and some fresh provisions. It cost us some gold, but it's worth it for the firepower!"));
    } else {
        actions.delta_crew(-1);
        actions.delta_food(10);
        actions.add_dialogue(captain!("We didn't have enough gold to trade properly. The natives took one of our crew as payment, but at least they gave us some food."));
    }
}

fn leave_island(actions: &mut StoryActions) {
    actions.delta_food(5);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We decided to play it safe and leave the island. The brief rest has done the crew some good, and we found a few coconuts to add to our supplies."));
}

pub fn the_hidden_cove_treasure_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! We've spotted a hidden cove on this uncharted island!"))
        .line(captain!("Interesting... What can you see from here?"))
        .line(crew2!("There's a cave entrance near the beach, and I think I saw some movement inland - could be natives."))
        .line(crew3!("The cave looks promising for treasure, but who knows what dangers lurk inside..."))
        .line(captain!("We've got a decision to make, lads. What shall it be?"))
        .choice("Explore Cave", explore_cave)
        .conditional_choice("Negotiate", negotiate_with_natives, actions.get_item(Item::Gold) >= 100)
        .choice("Leave", leave_island)
        .hint("Squawk! Fortune favors the bold, but sometimes caution is the better part of valor!")
}