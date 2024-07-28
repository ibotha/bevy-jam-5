use rand::Rng;
use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn enter_regatta(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture: _,
    } = actions.weather();

    actions.delta_items(Item::Gold, -50); // Entry fee

    match (heat, wind) {
        (H::Comfortable, W::Medium) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(5);
            actions.add_dialogue(captain!("Perfect conditions! We won the regatta, claiming the grand prize and some structural improvements from the shipwright."));
        }
        (H::Warm, W::Low | W::Medium) => {
            actions.delta_items(Item::Gold, 75);
            actions.delta_crew(1);
            actions.add_dialogue(captain!("We came in second place. The prize money is nice, and a skilled sailor was impressed enough to join our crew."));
        }
        (H::Blistering, _) | (_, W::GaleForce) => {
            actions.delta_health(-5);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("Disastrous weather! We had to withdraw from the race, damaging the ship and losing a crew member in the process."));
        }
        _ => {
            actions.delta_items(Item::Gold, 50);
            actions.add_dialogue(captain!("We finished the race, but didn't place. At least we got our entry fee back and a small participation prize."));
        }
    }
}

fn sabotage_competitor(actions: &mut StoryActions) {
    if actions.get_crew() > 3 {
        actions.delta_items(Item::Gold, 20);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("Our sabotage was successful, but one crew member was caught and arrested. Still, we made off with a share of the prize money."));
    } else {
        actions.delta_items(Item::Gold, -5);
        actions.delta_health(-5);
        actions.add_dialogue(captain!("Without enough crew, our sabotage attempt was discovered. We lost our entry fee and had to make a quick escape, damaging the ship in the process."));
    }
}

fn bet_on_races(actions: &mut StoryActions) {
    let bet_amount = 50;
    if actions.get_item(Item::Gold) >= bet_amount {
        actions.delta_items(Item::Gold, -bet_amount);

        if actions.get_rng().gen_bool(0.6) { // 60% chance to win
            actions.delta_items(Item::Gold, bet_amount * 2);
            actions.add_dialogue(captain!("Our bet paid off! We doubled our money without risking the ship."));
        } else {
            actions.add_dialogue(captain!("Seems luck wasn't on our side today. We lost our bet, but at least we enjoyed watching the races."));
        }
    } else {
        actions.delta_crew(1);
        actions.add_dialogue(captain!("We didn't have enough gold to place a bet, but one of our crew won some money gambling and decided to share with the ship."));
    }
}

pub fn the_ports_grand_regatta_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! The port is hosting its annual Grand Regatta today. Ships from all over are competing!"))
        .line(captain!("The Grand Regatta, eh? That could be interesting. What are the details?"))
        .line(crew2!("There's a hefty prize for the winner, Cap'n. But the entry fee ain't cheap, and the competition looks fierce."))
        .line(crew3!("We could always try to... influence the outcome in other ways, if you catch my drift."))
        .line(captain!("I see. And I suppose we could always just enjoy the spectacle. What are our options?"))
        .conditional_choice("Enter", enter_regatta, actions.get_item(Item::Gold) >= 50)
        .conditional_choice("Sabotage", sabotage_competitor, actions.get_crew() > 3)
        .choice("Bet", bet_on_races)
        .hint("Squawk! Fair winds make champions, but a clever captain always has an ace up their sleeve!")
}
