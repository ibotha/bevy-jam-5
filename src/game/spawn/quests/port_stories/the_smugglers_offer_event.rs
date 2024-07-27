use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn accept_smuggling_job(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture: _,
    } = actions.weather();

    match wind {
        W::None | W::Low => {
            actions.delta_items(Item::Gold, 200);
            actions.add_dialogue(captain!("Smooth sailing! We've made a tidy profit from this venture."));
        }
        W::Medium => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We completed the job, but the choppy waters caused some minor damage."));
        }
        W::High | W::GaleForce => {
            actions.delta_items(Item::Gold, -50);
            actions.delta_health(-15);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("Blasted storm! We lost the cargo and barely made it back alive."));
        }
    }

    // actions.add_event(FollowingEvent {
    //     environment: Environment::Sea,
    //     distance: 15,
    //     event: bounty_hunters(),
    //     certainty: Certainty::Possible(3),
    // });
}

fn reject_smuggling_job(actions: &mut StoryActions) {
    actions.delta_crew(1);
    actions.delta_items(Item::Gold, -20);
    actions.add_dialogue(captain!("We may have lost some coin, but we gained a new crew member who respects our integrity."));
}

fn bribe_smuggler(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 50 {
        actions.delta_items(Item::Gold, -50);
        actions.add_dialogue(captain!("A necessary expense to keep our reputation clean."));
    } else {
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("We couldn't pay, so they took one of our crew as collateral. We'll get them back... someday."));
    }
}

pub fn the_smugglers_offer_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, a shady character approached me at the docks with an... interesting proposition."))
        .line(captain!("Out with it, Patchy. What's this proposition?"))
        .line(crew1!("They're offering a hefty sum of gold to transport some, er, 'undocumented goods' to the next port."))
        .line(crew2!("Sounds risky, Cap'n. Could be trouble with the authorities."))
        .line(crew3!("But think of the gold! We could use it for repairs and supplies."))
        .line(captain!("Hmm, a tough decision indeed. What are our options?"))
        .choice("Accept", accept_smuggling_job)
        .choice("Reject", reject_smuggling_job)
        .conditional_choice("Bribe", bribe_smuggler, actions.get_item(Item::Gold) >= 50)
        .hint("Squawk! Honest sails sleep better at night!")
}