use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn navigate_carefully(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::None | W::Low, M::Humid) => {
            actions.delta_items(Item::Gold, 50);
            actions.add_dialogue(captain!("Our caution paid off! We discovered a small, uncharted island and found some valuable trinkets."));
        }
        (W::Medium, _) => {
            actions.delta_health(-5);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("We managed to navigate safely, but it took longer than expected. Our supplies have dwindled."));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_health(-15);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("The combination of fog and strong winds was treacherous! We sustained damage and lost a crew member overboard."));
        }
        _ => {
            actions.delta_food(-5);
            actions.add_dialogue(captain!("We made it through the fog without incident, but it was slow going. Used up more supplies than I'd like."));
        }
    }
}

fn use_mystical_compass(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_items(Item::Gold, 100);
        actions.delta_health(10);
        actions.add_dialogue(captain!("The Monkey's Paw guided us through the fog to a hidden treasure! But I can't shake the feeling we've invited some sort of curse..."));
        // You might want to add a negative effect later due to using the Monkey's Paw
    } else {
        actions.delta_health(-10);
        actions.add_dialogue(captain!("Without any mystical aid, we got turned around in the fog. Took some damage before finding our way out."));
    }
}

fn wait_it_out(actions: &mut StoryActions) {
    actions.delta_food(-15);
    actions.delta_items(Item::Gold, -25);
    actions.add_dialogue(captain!("We dropped anchor and waited for the fog to clear. Safe, but it cost us time and supplies. Missed out on some trading opportunities too."));
}

pub fn the_mysterious_fog_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!(
            "Cap'n! A strange fog's rollin' in. Never seen anythin' like it!"
        ))
        .line(captain!("Hmm, this is unusual. What can you see?"))
        .line(crew2!(
            "Not much, sir. It's thick as pea soup and seems to be glowin' slightly."
        ))
        .line(crew3!(
            "I've heard tales of magical fogs that hide great treasures... or terrible dangers."
        ))
        .line(captain!(
            "We need to make a decision. What's our course of action?"
        ))
        .choice("Navigate", navigate_carefully)
        .conditional_choice(
            "Use Compass",
            use_mystical_compass,
            actions.get_item(Item::MonkeyPaw) > 0,
        )
        // .choice("Wait", wait_it_out)
        .hint("Squawk! Sometimes the safest path leads to the greatest reward!")
}

