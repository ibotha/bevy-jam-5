use crate::game::spawn::quests::prelude::*;

fn mysterious_merchant(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    actions.add_dialogue(crew1!("Captain! There's a strange merchant in the port."));
    actions.add_dialogue(crew2!("Aye, his wares look... peculiar."));

    match (heat, moisture, wind) {
        (H::Blistering | H::Warm, M::Dry, W::None | W::Low) => {
            actions.add_dialogue(captain!("This heat is making me suspicious. Let's be cautious."));
            if actions.get_item(Item::Gold) > 50
            {
                actions.delta_items(Item::Gold, -50);
                actions.delta_items(Item::Cannon, 1);
                actions.add_dialogue(crew3!("We got a cannon, but it cost us more than usual."));
            }
        }
        (H::Comfortable, M::Comfortable, _) => {
            actions.add_dialogue(captain!("Perfect weather for a deal. Let's see what he's got."));
            if actions.get_item(Item::Gold) > 100
            {
                actions.delta_items(Item::Gold, -100);
                actions.delta_items(Item::MonkeyPaw, 1);
                actions.add_dialogue(crew1!("A Monkey's Paw? That's... interesting, Cap'n."));
            }
        }
        (H::Chilly | H::Freezing, _, W::High | W::GaleForce) => {
            actions.add_dialogue(captain!("This cold wind's putting me off. We should leave."));
            if actions.get_item(Item::Gold) > 50
            {
                actions.delta_crew(-1);
                actions.delta_items(Item::Gold, 50);
                actions.add_dialogue(crew2!("One of our crew got spooked and left, but we found some gold!"));
            }
            else {
                actions.add_dialogue(crew3!("We didn't find anything worth our time."));
            }
        }
        _ => {
            actions.add_dialogue(captain!("Nothing catches my eye. Let's move on."));
            actions.delta_food(5);
            actions.add_dialogue(crew3!("At least we got some grub from the market."));
        }
    }
}

pub fn the_mysterious_merchant_event() -> DayEvent {
    DayEvent::new()
        .line(captain!("Ahoy! What's all this commotion in the port?"))
        .line(crew1!("There's a peculiar merchant, Cap'n. Never seen him before."))
        .line(crew2!("His wares look... unusual. Might be worth checkin' out."))
        .line(crew3!("Or it could be trouble. What's your call, Cap'n?"))
        .choice("Investigate the merchant", mysterious_merchant)
        .choice("Ignore and continue", |actions| {
            actions.add_dialogue(captain!("We've no time for curiosities. Back to work!"));
            actions.delta_crew(1);
        })
        .hint("Squawk! Mysterious trades can be risky or rewarding!")
}
