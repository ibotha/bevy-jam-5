use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn enter_the_rift(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture: _,
    } = actions.weather();

    match (heat, wind) {
        (H::Comfortable | H::Warm, W::Low | W::Medium) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_health(20);
            actions.delta_crew(1);
            actions.add_dialogue(captain!("Incredible! We've returned with futuristic technology we can sell, our ship has been improved by future techniques, and we've even recruited a crew member from another time!"));
        }
        (H::Chilly | H::Freezing, W::Medium | W::High) => {
            actions.delta_crew(-1);
            actions.delta_items(Item::Gold, 20);
            actions.delta_food(50);
            actions.add_dialogue(captain!("We've lost a crew member to the past, but we've brought back valuable historical artifacts and a supply of extinct, delicious creatures. The crew is shaken but amazed."));
        }
        (H::Blistering, W::High | W::GaleForce) => {
            actions.delta_crew(-2);
            actions.delta_health(-25);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("The rift was unstable! We've lost two crew members and taken damage, but we managed to grab a strangely advanced cannon before escaping."));
        }
        _ => {
            actions.delta_items(Item::Gold, 15);
            actions.delta_health(10);
            actions.delta_food(20);
            actions.add_dialogue(captain!("We cautiously explored the rift and returned with some valuable trinkets from various times. The experience has left our ship slightly more resilient to temporal effects."));
        }
    }
}

fn study_the_anomaly(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(30);
        actions.delta_items(Item::Gold, 50);
        actions.add_dialogue(captain!("The Monkey's Paw resonated with the time rift! We've gained incredible insights into time itself, recruited two time-lost individuals, and our ship now seems to move faster than ever!"));
    } else {
        actions.delta_food(-10);
        actions.delta_health(-5);
        actions.delta_items(Item::Gold, 25);
        actions.add_dialogue(captain!("Our attempts to study the anomaly were partially successful. We've learned some interesting temporal tricks and found some valuable time-shifted objects, but the effort has left us drained."));
    }
}

fn avoid_the_anomaly(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We steered clear of the time rift. It took some extra supplies to go around, but the crew is relieved we avoided such a reality-bending phenomenon."));
}

pub fn the_time_rift_anomaly_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's something strange ahead... the sea looks... twisted!"))
        .line(captain!("Twisted? Explain yourself, sailor!"))
        .line(crew2!("It's like the water is folding in on itself, sir! And I swear I saw a galleon from the last century sail by!"))
        .line(crew3!("It must be a time rift, Cap'n! I've heard tales of these - windows to the past and future that sometimes appear at sea!"))
        .line(captain!("A time rift? This could be our most bizarre adventure yet. What's our move, crew?"))
        .choice("Enter Rift", enter_the_rift)
        .conditional_choice("Study Anomaly", study_the_anomaly, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Avoid", avoid_the_anomaly)
        .hint("Squawk! Time waits for no man... unless you find a rift!")
}