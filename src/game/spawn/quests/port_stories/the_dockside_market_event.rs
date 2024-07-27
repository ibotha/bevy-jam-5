use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;
use rand::Rng;

fn purchase_supplies(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 50 {
        actions.delta_items(Item::Gold, -50);
        actions.delta_food(15);
        actions.delta_health(5);
        actions.add_dialogue(captain!("We stocked up on fresh provisions and got some materials for minor repairs. The crew's spirits are lifted."));
    } else {
        actions.delta_items(Item::Gold, -10);
        actions.delta_food(5);
        actions.add_dialogue(captain!(
            "We could only afford some basic supplies, but it's better than nothing."
        ));
    }
}

fn sell_excess_cargo(actions: &mut StoryActions) {
    let gold_gain = actions.get_rng().gen_range(30..=80);
    actions.delta_items(Item::Gold, gold_gain);
    actions.delta_food(-5);
    actions.add_dialogue(captain!(format!(
        "We sold some of our excess cargo for {} gold. It's freed up some space in our hold.",
        gold_gain
    )));
}

fn recruit_new_crew(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 30 {
        actions.delta_items(Item::Gold, -30);
        actions.delta_crew(1);
        actions.add_dialogue(captain!(
            "We found a capable sailor looking for work. They've joined our crew for a modest fee."
        ));
    } else {
        actions.add_dialogue(captain!("We spotted some potential recruits, but couldn't afford to hire anyone new at the moment."));
    }
}

pub fn the_dockside_market_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, the dockside market is bustling today. Shall we see what's on offer?"))
        .line(captain!("Aye, it's always worth a look. What are our options?"))
        .line(crew2!("We could stock up on supplies, sell some of our excess cargo, or look for new crew members."))
        .line(crew3!("The prices seem fair today, Cap'n. Might be a good opportunity."))
        .conditional_choice("Purchase", purchase_supplies, actions.get_item(Item::Gold) >= 10)
        .choice("Sell", sell_excess_cargo)
        .conditional_choice("Recruit", recruit_new_crew, actions.get_item(Item::Gold) >= 30)
        .hint("Squawk! A well-stocked ship makes for happy sailing!")
}

