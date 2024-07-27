use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;

fn accept_diplomatic_mission(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 150);
    actions.delta_food(-10);
    actions.delta_health(-5);
    actions.add_dialogue(captain!("We've delivered the diplomat safely. The journey was taxing, but the pay was worth it. We've also gained some useful political connections."));
    // We can potentially add a future event
}

fn decline_politely(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 25);
    actions.add_dialogue(captain!("We politely declined the offer, but the diplomat appreciated our honesty. He gave us a small token of goodwill for our time."));
}

fn offer_information(actions: &mut StoryActions) {
    if actions.get_crew() > 5 {
        actions.delta_items(Item::Gold, 50);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("We shared what we knew about the seas. The diplomat was grateful and rewarded us, but one of our crew was hired away as a local guide."));
    } else {
        actions.delta_items(Item::Gold, 30);
        actions.add_dialogue(captain!("We offered what limited information we could. The diplomat seemed satisfied and gave us a modest reward."));
    }
}

pub fn the_foreign_diplomats_request_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! There's a foreign diplomat at the docks asking for you specifically."))
        .line(captain!("A diplomat? What could they want with us?"))
        .line(crew2!("Seems they're in need of discreet transportation, Cap'n. Offering good coin too."))
        .line(crew3!("But it could be risky getting involved in political affairs. They're also interested in information about the local waters."))
        .line(captain!("Interesting... What are our options here?"))
        .choice("Accept", accept_diplomatic_mission)
        .choice("Decline", decline_politely)
        .conditional_choice("Offer info", offer_information, actions.get_crew() > 3)
        .hint("Squawk! Politics and piracy make strange bedfellows, but gold spends the same!")
}

