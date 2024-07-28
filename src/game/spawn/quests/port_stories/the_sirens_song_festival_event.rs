use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;

fn participate_in_festival(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable | H::Warm, W::Low | W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, -5);
            actions.delta_crew(2);
            actions.delta_health(10);
            actions.add_dialogue(captain!("The festival was a delight! We've gained some skilled musicians as crew, and the ship seems to have a new vigor."));
        }
        (H::Blistering, _, M::Humid) | (_, W::GaleForce, _) => {
            actions.delta_items(Item::Gold, -10);
            actions.delta_crew(-1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The harsh weather turned the festival sour. We lost money and a crew member to the siren's song."));
        }
        _ => {
            actions.delta_items(Item::Gold, -5);
            actions.delta_food(15);
            actions.add_dialogue(captain!("The festival was decent. We didn't gain much, but the crew's spirits are high and we got some unique provisions."));
        }
    }
}

fn seek_siren_wisdom(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 15 {
        actions.delta_items(Item::Gold, -15);
        actions.delta_health(15);
        actions.delta_food(-10);
        actions.add_dialogue(captain!("The siren's wisdom was costly but valuable. Our ship feels stronger, though we used up some provisions in the process."));
    } else {
        actions.delta_items(Item::Gold, -5);
        actions.delta_crew(-1);
        actions.delta_food(5);
        actions.add_dialogue(captain!("We couldn't afford the full wisdom, but the siren gave us some cryptic advice. We lost a crew member to her song, but gained some magical food."));
    }
}

fn avoid_festival(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 5);
    actions.delta_crew(-1);
    actions.add_dialogue(captain!("We stayed away from the mysterious festival. It saved us some trouble, but one crew member left, drawn by the siren's song. At least we made some money from other cautious traders."));
}

pub fn the_sirens_song_festival_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! The village is all abuzz about some festival tonight. They call it the Siren's Song Festival."))
        .line(captain!("A Siren's Song Festival? Sounds dangerous..."))
        .line(crew2!("Aye, but they say it only happens once every seven years. Could be a unique experience, Cap'n."))
        .line(crew3!("I've heard whispers that those who participate might gain special boons... or terrible curses."))
        .line(captain!("Hmm, intriguing. What are our options?"))
        .conditional_choice("Participate", participate_in_festival, actions.get_item(Item::Gold) >= 50)
        .conditional_choice("Seek wisdom", seek_siren_wisdom, actions.get_item(Item::Gold) >= 50)
        .choice("Avoid festival", avoid_festival)
        .hint("Squawk! Sometimes the sweetest songs hide the sharpest teeth!")
}

