use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn salvage_wreck(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Dry) => {
            actions.delta_items(Item::Gold, 30);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("Arr! The conditions were perfect for salvage. We've recovered a hefty sum of gold and even a working cannon!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 15);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We managed to salvage some gold, but the shifting wreck made it dangerous. The ship took some minor damage in the process."));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("The weather turned foul during our salvage attempt. We lost a crewman to the treacherous waves and the ship took a beating."));
        }
        _ => {
            actions.delta_items(Item::Gold, 10);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("We recovered some gold, but the effort took longer than expected. We've used up more of our food supplies than I'd like."));
        }
    }
}

fn explore_island(actions: &mut StoryActions) {
    actions.delta_food(20);
    actions.delta_health(5);
    actions.delta_items(Item::Gold, -50);
    actions.add_dialogue(captain!("We found fresh water and fruit on the island, replenishing our supplies. However, we spent some coin on equipment for the exploration."));
}

pub fn the_mysterious_shipwreck_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!(
            "Cap'n! There's a shipwreck on the beach of that island ahead!"
        ))
        .line(captain!(
            "Interesting... Any signs of survivors or other ships in the area?"
        ))
        .line(crew2!(
            "No signs of life, Cap'n. The wreck looks old, might be worth salvaging."
        ))
        .line(crew3!(
            "The island itself looks lush. Could be a good spot to replenish our supplies."
        ))
        .line(captain!("We've got options, crew. What's our move?"))
        .choice("Salvage", salvage_wreck)
        .choice("Explore", explore_island)
        .hint("Squawk! Every wreck has a story, and sometimes a fortune!")
}

