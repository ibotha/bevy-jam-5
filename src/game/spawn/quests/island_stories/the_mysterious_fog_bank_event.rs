use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn navigate_fog(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Dry) => {
            actions.delta_items(Item::Gold, 30);
            actions.delta_health(10);
            actions.add_dialogue(captain!("We've done it! Navigated through the fog and found a hidden cove. There's abandoned pirate loot here, and the journey somehow invigorated our ship!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 15);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("We found some treasure, but the fog was disorienting. We... we lost a man overboard. May the sea have mercy on his soul."));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_health(-15);
            actions.delta_crew(-2);
            actions.add_dialogue(captain!("Blasted fog! We've run aground on hidden reefs. The ship's taken heavy damage, and we lost two good men to the treacherous waters!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 10);
            actions.delta_food(-20);
            actions.add_dialogue(captain!("We navigated the fog, finding a small treasure along the way. But it took longer than expected, and our food supplies have dwindled."));
        }
    }
}

fn use_cannon(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 1 {
        actions.delta_items(Item::Cannon, -1);
        actions.delta_items(Item::Gold, 50);
        actions.add_dialogue(captain!("Brilliant idea! The cannon fire dispersed the fog, revealing a merchant ship. They surrendered without a fight, and we've taken their gold!"));
    } else {
        actions.delta_health(-5);
        actions.add_dialogue(captain!("We don't have any cannons, ye blithering idiot! We'll have to brave the fog without them. The crew's morale has taken a hit from this foolish order."));
    }
}

fn avoid_fog(actions: &mut StoryActions) {
    actions.delta_food(15);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We've steered clear of the fog, finding a small, clear lagoon instead. The crew's caught some fish and had a chance to rest. It's not gold, but it'll do."));
}

pub fn the_mysterious_fog_bank_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a massive fog bank ahead!"))
        .line(captain!("Hmm, that's not on any of our charts. What do you make of it?"))
        .line(crew2!("It's unnatural, Cap'n. I've heard tales of hidden treasures in such fogs, but also of ships never returning."))
        .line(crew3!("We could try to use our cannon to clear it, if we have any. The noise might disperse the fog."))
        .line(captain!("This could be an opportunity or a grave mistake. What's our course of action?"))
        .choice("Navigate Fog", navigate_fog)
        .conditional_choice("Use Cannon", use_cannon, actions.get_item(Item::Cannon) >= 1)
        //.choice("Avoid Fog", avoid_fog)
        .hint("Squawk! Sometimes the greatest treasures hide behind the thickest veils!")
}

