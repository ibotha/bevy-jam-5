use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn rescue_sailor(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Comfortable) => {
            actions.delta_crew(1);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("The rescue went smoothly. The sailor's grateful and has joined our crew. He's famished though, so we've shared some of our provisions."));
        }
        (W::Medium, M::Humid) => {
            actions.delta_crew(1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We managed to rescue the sailor, but the humid weather made it tricky. The ship took some minor damage from the rocks near shore."));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_health(-10);
            actions.delta_food(-20);
            actions.add_dialogue(captain!("The rescue was perilous in this wind! We barely made it back to the ship. The sailor's safe, but we used a lot of supplies and the ship's worse for wear."));
        }
        _ => {
            actions.delta_crew(1);
            actions.delta_items(Item::Gold, -50);
            actions.add_dialogue(captain!("We've rescued the sailor, but he insisted on compensation for his 'valuable information' about these waters. Seems a bit steep, if you ask me."));
        }
    }
}

fn trade_with_sailor(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -100);
        actions.delta_items(Item::Cannon, 1);
        actions.add_dialogue(captain!("The sailor had a spare cannon he was willing to part with. It's a bit pricey, but it'll serve us well in future battles."));
    } else {
        actions.delta_items(Item::Gold, -50);
        actions.delta_food(30);
        actions.add_dialogue(captain!("We didn't have enough gold for the cannon, but we managed to trade for some of his preserved food supplies. It'll keep our crew fed for a while."));
    }
}

fn ignore_sailor(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 25);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We left the sailor to his own devices. As we sailed away, we spotted a small treasure chest washed up on the shore. A stroke of luck!"));
}

pub fn the_stranded_sailor_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a man waving at us from the beach!"))
        .line(captain!("Interesting... Can you make out any details?"))
        .line(crew2!("Looks like a sailor, Cap'n. Probably shipwrecked. He seems to have some supplies with him."))
        .line(crew3!("Could be a trap, or he might have valuable information about these waters."))
        .line(captain!("We've got a decision to make, lads. What shall we do?"))
        .choice("Rescue", rescue_sailor)
        .conditional_choice("Trade", trade_with_sailor, actions.get_item(Item::Gold) >= 50)
        .choice("Ignore", ignore_sailor)
        .hint("Squawk! A helping hand today might save your own tomorrow!")
}