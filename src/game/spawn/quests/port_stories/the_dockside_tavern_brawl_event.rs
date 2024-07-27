use crate::game::spawn::quests::prelude::*;

fn handle_tavern_brawl(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    actions.add_dialogue(crew1!("Cap'n! A fight's broken out in the tavern!"));
    actions.add_dialogue(crew2!("It's getting ugly fast. What should we do?"));

    match (heat, moisture, wind) {
        (H::Blistering | H::Warm, M::Humid, _) => {
            actions.add_dialogue(captain!("This heat and humidity are making tempers flare. Let's try to calm things down."));
            if actions.get_item(Item::Gold) > 20 {
                actions.delta_items(Item::Gold, -20);
                actions.delta_crew(2);
                actions.add_dialogue(crew3!("We bought a round for the house. It cost us some gold, but we gained two new crew members!"));
            } else {
                actions.delta_crew(-1);
                actions.add_dialogue(crew1!("We couldn't afford to calm the situation. One of our crew got caught in the crossfire."));
            }
        }
        (H::Comfortable, _, W::None | W::Low) => {
            actions.add_dialogue(captain!("Calm weather means clear heads. Let's see if we can turn this to our advantage."));
            if actions.get_item(Item::Cannon) > 0 {
                actions.delta_items(Item::Cannon, -1);
                actions.delta_items(Item::Gold, 100);
                actions.add_dialogue(crew2!("We sold our spare cannon to the tavern owner for protection. Made a tidy profit!"));
            } else {
                actions.delta_food(10);
                actions.add_dialogue(crew3!("No cannon to sell, but we managed to swipe some food in the chaos."));
            }
        }
        (H::Chilly | H::Freezing, _, W::High | W::GaleForce) => {
            actions.add_dialogue(captain!("This wild weather's got everyone on edge. We'd best stay out of it."));
            actions.delta_crew(-2);
            actions.delta_items(Item::MonkeyPaw, 1);
            actions.add_dialogue(crew1!("Two of our crew got arrested in the brawl, but look what I found in the gutter - a Monkey's Paw!"));
        }
        _ => {
            actions.add_dialogue(captain!("Just another day at port. Let's keep our distance and watch the show."));
            actions.delta_food(-5);
            actions.add_dialogue(crew2!("We lost some grub in the commotion, but at least we're all in one piece."));
        }
    }
}

pub fn the_dockside_tavern_brawl_event() -> DayEvent {
    DayEvent::new()
        .line(captain!("Ahoy! What's all that ruckus coming from the tavern?"))
        .line(crew1!("Looks like a fight's broken out, Cap'n. It's getting pretty heated."))
        .line(crew2!("Could be trouble... or opportunity. What's our move?"))
        .line(crew3!("We could try to break it up, join in, or just steer clear."))
        .choice("Try to calm the situation", handle_tavern_brawl)
        .choice("Walk away and avoid trouble", |actions| {
            actions.add_dialogue(captain!("Not our circus, not our monkeys. Back to the ship!"));
            actions.delta_crew(-1);
            actions.add_dialogue(crew1!("Wise choice, Cap'n. Though we lost one crew member who couldn't resist the brawl."));
        })
        .hint("Squawk! A tavern brawl can be dangerous, but sometimes there's treasure in chaos!")
}