use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn accept_challenge(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 300 && actions.get_crew() >= 7 {
        actions.delta_items(Item::Gold, -300);
        actions.delta_crew(-2);
        actions.delta_health(-15);
        actions.delta_items(Item::Gold, 1000);
        actions.delta_items(Item::Cannon, 3);
        actions.add_dialogue(captain!("By the stars, we've done it! We've completed the Pirate King's Challenge and claimed the legendary treasure. We lost two brave souls, but our names will be remembered for generations!"));
        // You could add a special flag or item to commemorate this achievement
    } else {
        actions.delta_items(Item::Gold, -100);
        actions.delta_health(-10);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("We weren't prepared for such a trial. We failed the challenge and barely escaped with our lives. The crew's morale has taken a hit."));
    }
}

fn decipher_clues(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -50);
    actions.delta_food(-10);
    actions.delta_items(Item::Gold, 200);
    actions.add_dialogue(captain!("We spent days poring over the clues and charts. While we didn't attempt the challenge itself, we discovered the location of a smaller treasure cache. The knowledge gained could be invaluable for future adventures."));
    // You could add a future benefit related to treasure hunting or navigation
}

fn sell_information(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 350);
    actions.delta_crew(-1);
    actions.add_dialogue(captain!("We sold the information to a consortium of wealthy merchants. The payment was substantial, but one of our crew left, disgusted by our 'betrayal' of pirate honor."));
}

pub fn the_ancient_pirate_kings_challenge_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! You'll never believe what's just been announced in the port!"))
        .line(captain!("Calm down and speak plainly. What's got you so excited?"))
        .line(crew2!("It's the Challenge of the Ancient Pirate King, Cap'n! A series of trials leading to his legendary treasure hoard!"))
        .line(crew3!("Aye, but it's said to be incredibly dangerous. Many have tried and failed, losing their ships and crews in the process."))
        .line(captain!("The Pirate King's Challenge, eh? This could be our chance for eternal glory... or a quick trip to Davy Jones' locker. What are our options?"))
        .conditional_choice("Accept Challenge", accept_challenge, actions.get_item(Item::Gold) >= 100 && actions.get_crew() >= 7)
        .choice("Decipher", decipher_clues)
        .choice("Sell", sell_information)
        .hint("Squawk! Only the bravest or the foolhardiest dare to challenge the legends of old!")
}