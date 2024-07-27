use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn investigate_lighthouse(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Dry) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(10);
            actions.add_dialogue(captain!("We discovered an ancient artifact causing the curse. Removing it lifted the curse and earned us a reward. The ship feels oddly invigorated."));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_crew(1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We encountered a trapped spirit. After freeing it, it joined our crew! However, the ordeal left our ship slightly damaged."));
        }
        (W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-2);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("The curse was too powerful! We barely escaped, losing two crew members and damaging the ship in the process."));
        }
        _ => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_food(-5);
            actions.add_dialogue(captain!("We found the source of the curse but couldn't fully lift it. The lighthouse keeper rewarded our effort, though we used up some provisions."));
        }
    }
}

fn hire_exorcist(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -100);
        actions.delta_health(15);
        actions.delta_food(10);
        actions.add_dialogue(captain!("The exorcist successfully lifted the curse. Our ship feels stronger, and we received some blessed provisions as thanks."));
    } else {
        actions.delta_items(Item::Gold, -50);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("We couldn't afford the full service. The exorcist partially lifted the curse, but one of our crew was possessed in the process."));
    }
}

fn ignore_lighthouse(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -25);
    actions.delta_food(5);
    actions.add_dialogue(captain!("We avoided the cursed lighthouse, but navigation became trickier. We lost some time and gold, but the locals appreciated our caution and gave us some supplies."));
}

pub fn the_cursed_lighthouse_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! The port's lighthouse has gone dark, and there are whispers of a curse!"))
        .line(captain!("A cursed lighthouse? That's a new one. What are people saying?"))
        .line(crew2!("They say strange noises come from it at night, and ships are veering off course, Cap'n."))
        .line(crew3!("The harbor master is offering a reward to anyone who can solve the problem."))
        .line(captain!("Interesting... What are our options?"))
        .choice("Investigate", investigate_lighthouse)
        .conditional_choice("Exorcist", hire_exorcist, actions.get_item(Item::Gold) >= 100)
        .choice("Ignore", ignore_lighthouse)
        .hint("Squawk! Even the darkest curses can lead to bright opportunities!")
}