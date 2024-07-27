use rand::Rng;
use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn mediate_dispute(actions: &mut StoryActions) {
    let success = actions.get_rng().gen_bool(0.6); // 60% chance of successful mediation
    if success {
        actions.delta_items(Item::Gold, 50);
        actions.delta_crew(1);
        actions.add_dialogue(captain!("Our diplomacy paid off. The dispute is resolved, we earned a reward, and one of the dockworkers decided to join our crew!"));
    } else {
        actions.delta_health(-5);
        actions.add_dialogue(captain!("Our attempt to mediate backfired. We got caught in the middle of an argument and took some minor damage in the scuffle."));
    }
}

fn hire_alternative_workers(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 75 {
        actions.delta_items(Item::Gold, -75);
        actions.delta_food(10);
        actions.delta_health(5);
        actions.add_dialogue(captain!("We hired some temp workers to load our supplies. It cost extra, but we got our work done efficiently and even managed some minor repairs."));
    } else {
        actions.delta_items(Item::Gold, -25);
        actions.delta_food(5);
        actions.add_dialogue(captain!("We could only afford a skeleton crew. They loaded some basic supplies, but it's less than we hoped for."));
    }
}

fn wait_out_dispute(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_items(Item::Gold, 20);
    actions.add_dialogue(captain!("We decided to wait it out. Lost some time and had to use some of our provisions, but we saved on docking fees and sold some goods to other waiting ships."));
}

pub fn the_dockworkers_dispute_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n, there's a bit of a situation on the docks. The workers are in a dispute with the harbor master."))
        .line(captain!("A dispute, you say? What's the nature of it?"))
        .line(crew2!("Something about wages and working conditions, Cap'n. It's causing delays for all the ships in port."))
        .line(crew3!("We need to resupply and do some trading. This could be a problem if it's not resolved soon."))
        .line(captain!("I see. What are our options for dealing with this?"))
        .choice("Dispute", mediate_dispute)
        .conditional_choice("Hire", hire_alternative_workers, actions.get_item(Item::Gold) >= 25)
        .choice("Wait", wait_out_dispute)
        .hint("Squawk! A smooth sea never made a skilled sailor, nor a busy port a dull day!")
}