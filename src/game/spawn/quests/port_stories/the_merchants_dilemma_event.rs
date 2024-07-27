use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn escort_merchant(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_food(10);
            actions.add_dialogue(captain!("Smooth sailing! We safely escorted the merchant and earned a handsome reward. They even shared some of their exotic foods with us."));
        }
        (W::High, M::Dry) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The journey was rough, but we managed to protect the merchant. Our ship took some damage, but the pay was worth it."));
        }
        (W::GaleForce, _) | (_, M::Humid) => {
            actions.delta_items(Item::Gold, -50);
            actions.delta_crew(-1);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("Disaster struck! We lost the merchant's cargo, a crew member, and our ship took heavy damage. We had to compensate the merchant for the loss."));
        }
        _ => {
            actions.delta_items(Item::Gold, 150);
            actions.delta_crew(1);
            actions.add_dialogue(captain!("We completed the escort successfully. The pay was good, and one of the merchant's guards was so impressed they decided to join our crew."));
        }
    }
}

fn buy_cargo(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 300 {
        actions.delta_items(Item::Gold, -300);
        actions.delta_items(Item::Cannon, 2);
        actions.add_dialogue(captain!("We bought the merchant's cargo at a bargain price. These cannons will serve us well in future battles!"));
    } else {
        actions.delta_items(Item::Gold, -100);
        actions.delta_food(20);
        actions.add_dialogue(captain!("We couldn't afford the full cargo, but we managed to buy a good amount of provisions at a discounted price."));
    }
}

fn decline_assistance(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 50);
    actions.delta_crew(-1);
    actions.add_dialogue(captain!("We avoided the risk, but one of our crew left, disappointed by our lack of adventure. At least we made some money from other jobs at the port."));
}

pub fn the_merchants_dilemma_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! There's a merchant in a right state at the docks. Says he's in desperate need of help."))
        .line(captain!("A merchant, you say? What seems to be the trouble?"))
        .line(crew2!("His usual escort cancelled on him, Cap'n. He's got valuable cargo to move, but the waters are treacherous."))
        .line(crew3!("He's offering good coin for an escort, but he's also willing to sell his cargo at a discount if someone buys it outright."))
        .line(captain!("Interesting... What are our options?"))
        .choice("Escort", escort_merchant)
        .conditional_choice("Buy", buy_cargo, actions.get_item(Item::Gold) >= 100)
        .choice("Decline", decline_assistance)
        .hint("Squawk! A merchant's problem can be a pirate's opportunity!")
}