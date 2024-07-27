use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn explore_outpost(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 400);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("Jackpot! We've found a hidden stash of gold and a well-preserved cannon. The dry weather kept everything in good condition."));
        }
        (H::Warm, W::Medium, M::Humid) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("We found some gold, but the humid air made the old structures unstable. A partial collapse damaged our ship. We're lucky it wasn't worse."));
        }
        (H::Blistering, _, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("Disaster! The extreme conditions triggered old traps in the outpost. We lost two crew members and the ship took a beating. There was no treasure to be found, curse our luck!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 150);
            actions.delta_food(20);
            actions.add_dialogue(captain!("We found a modest sum of gold and some preserved rations. Not a bad haul, all things considered."));
        }
    }
}

fn recruit_marooned_pirate(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -100);
        actions.delta_crew(1);
        actions.delta_health(5);
        actions.add_dialogue(captain!("We've convinced the old sea dog to join us. Cost us some gold, but his knowledge of these waters could prove invaluable. The crew's spirits are lifted by the new company."));
    } else {
        actions.delta_health(-5);
        actions.delta_food(-10);
        actions.add_dialogue(captain!("We didn't have enough gold to entice the pirate. He grudgingly shared some information about dangerous waters ahead, but it's cost us time and supplies."));
    }
}

fn salvage_materials(actions: &mut StoryActions) {
    actions.delta_health(15);
    actions.delta_food(-15);
    actions.add_dialogue(captain!("We've stripped the outpost for all it's worth. The materials have patched up our ship nicely, but the work has depleted some of our food supplies."));
}

pub fn the_abandoned_pirate_outpost_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's an old pirate outpost on this island!"))
        .line(captain!("Interesting find, lad. What's its state?"))
        .line(crew2!("Looks abandoned, Cap'n, but there might be treasures left behind. Or traps."))
        .line(crew3!("I think I saw movement inside. Could be a marooned pirate, or just the wind playing tricks."))
        .line(captain!("An intriguing opportunity, crew. What shall we do?"))
        .choice("Explore Outpost", explore_outpost)
        .conditional_choice("Recruit Pirate", recruit_marooned_pirate, actions.get_item(Item::Gold) >= 100)
        .choice("Salvage Materials", salvage_materials)
        .hint("Squawk! One pirate's trash is another pirate's treasure!")
}