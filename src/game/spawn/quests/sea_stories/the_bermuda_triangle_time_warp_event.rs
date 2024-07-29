use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn navigate_time_streams(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_health(25);
            actions.delta_crew(2);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("Incredible! We've mastered the time streams! Our ship now exists in multiple time periods simultaneously, we've recruited legendary figures from history, and our holds are bursting with treasures from the past and future!"));
        }
        (H::Warm | H::Chilly, W::Low | W::High, M::Dry | M::Humid) => {
            actions.delta_items(Item::Gold, 150);
            actions.delta_health(-5);
            actions.delta_crew(-1);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("We've escaped the time warp, but at a cost. Five crew members were lost to different eras, our ship shows signs of both aging and futuristic upgrades, and we've acquired anachronistic treasures and weapons."));
        }
        (H::Blistering | H::Freezing, W::GaleForce, _) => {
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.delta_items(Item::Gold, 80);
            actions.add_dialogue(captain!("The time streams nearly tore us apart! We've lost ten crew members across time, our ship is a patchwork of different eras, but we've managed to salvage some extraordinary artifacts from various time periods."));
        }
        _ => {
            actions.delta_items(Item::Gold, 120);
            actions.delta_health(10);
            actions.delta_food(10);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("We navigated the time anomaly cautiously. Our ship now flickers between past and future states, we've gathered provisions from multiple eras, and acquired some advanced weaponry from the future."));
        }
    }
}

fn attempt_temporal_mastery(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(20);
        actions.delta_items(Item::Gold, 250);
        actions.delta_items(Item::Cannon, 2);
        actions.add_dialogue(captain!("The Monkey's Paw resonated with the temporal energies! We've achieved mastery over time itself! Our crew now includes our own future selves, our ship can jump through time at will, and we've amassed a fortune from multiple timelines!"));
    } else {
        actions.delta_food(30);
        actions.delta_health(-10);
        actions.delta_items(Item::Gold, 70);
        actions.delta_crew(-2);
        actions.add_dialogue(captain!("Our attempt at temporal mastery was partially successful. We can now perceive multiple timelines, but the strain caused four crew members to age rapidly and disintegrate. We've collected valuable artifacts from various eras, but at a great cost."));
    }
}

fn ride_out_the_anomaly(actions: &mut StoryActions) {
    actions.delta_food(-20);
    actions.delta_health(-5);
    actions.delta_items(Item::Gold, 50);
    actions.add_dialogue(captain!("We chose to ride out the time anomaly, letting it pass naturally. The experience was disorienting - we saw our ship age and rejuvenate repeatedly. We've emerged with some time-shifted artifacts, but the crew is shaken and our supplies depleted from experiencing multiple timelines simultaneously."));
}

pub fn the_bermuda_triangle_time_warp_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The compass is spinning wildly and... is the sun moving backwards?!"))
        .line(captain!("What devilry is this? Report, man!"))
        .line(crew2!("We've sailed into a fog, sir, and now nothing makes sense! I swear I just saw a futuristic ship appear and disappear!"))
        .line(crew3!("It's a Bermuda Triangle time warp, Cap'n! We're caught in a temporal anomaly where past, present, and future collide!"))
        .line(captain!("A time warp in the Bermuda Triangle... This could be our greatest adventure or our ultimate doom. What's our course of action, crew?"))
        .choice("Navigate Time", navigate_time_streams)
        .conditional_choice("Master Time", attempt_temporal_mastery, actions.get_item(Item::MonkeyPaw) > 0)
        //.choice("Ride it Out", ride_out_the_anomaly)
        .hint("Squawk! Time is but a river, and we're about to ride the rapids!")
}

