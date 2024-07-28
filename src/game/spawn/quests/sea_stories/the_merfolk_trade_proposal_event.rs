use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn accept_trade(actions: &mut StoryActions) {
    let DW {
        heat,
        wind: _,
        moisture: _,
    } = actions.weather();

    match heat {
        H::Comfortable | H::Warm => {
            actions.delta_items(Item::Gold, -100);
            actions.delta_food(30);
            actions.delta_health(15);
            actions.add_dialogue(captain!("The merfolk were true to their word! We traded some gold for their enchanted kelp. Our food stores are full and the crew feels invigorated!"));
        }
        H::Chilly | H::Freezing => {
            actions.delta_items(Item::Gold, -100);
            actions.delta_food(20);
            actions.delta_crew(1);
            actions.add_dialogue(captain!("The trade was successful, but one of the merfolk decided to join our crew! They say they're curious about life on the surface."));
        }
        H::Blistering => {
            actions.delta_items(Item::Gold, -100);
            actions.delta_health(-10);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("The heat made the merfolk irritable. They gave us a rusty old cannon instead of food, claiming it's a 'surface dweller thing'. At least it still works!"));
        }
    }
}

fn negotiate_better_deal(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_items(Item::Gold, -50);
        actions.delta_food(40);
        actions.delta_health(20);
        actions.add_dialogue(captain!("The Monkey's Paw glowed during negotiations. We got double the enchanted kelp for half the price! The merfolk seemed entranced by the paw."));
    } else {
        actions.delta_items(Item::Gold, -150);
        actions.delta_food(25);
        actions.add_dialogue(captain!("Our attempt to negotiate backfired. The merfolk drove a harder bargain, but we still got the enchanted kelp."));
    }
}

fn decline_offer(actions: &mut StoryActions) {
    actions.delta_food(-10);
    actions.delta_items(Item::Gold, 50);
    actions.add_dialogue(captain!("We politely declined their offer. The merfolk seemed disappointed but gave us a small pearl as a gesture of goodwill before departing."));
}

pub fn the_merfolk_trade_proposal_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! You won't believe this, but there are merfolk alongside the ship!"))
        .line(captain!("Merfolk? Are you sure you haven't been drinking too much grog?"))
        .line(crew2!("It's true, sir! They're offering to trade some sort of enchanted kelp for gold."))
        .line(crew3!("They say it'll replenish our food stores and boost the crew's health, Cap'n."))
        .line(captain!("Interesting... What do you think we should do, crew?"))
        .choice("Accept Trade", accept_trade)
        .conditional_choice("Negotiate", negotiate_better_deal, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Decline", decline_offer)
        .hint("Squawk! Sometimes the sea itself offers the sweetest deals!")
}