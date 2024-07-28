use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn explore_atlantis(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable | H::Warm, W::Low | W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_health(30);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("Unbelievable! We've discovered Atlantean treasures beyond our wildest dreams. Our ship has been enhanced with their advanced technology, and we're rich beyond measure!"));
        }
        (H::Chilly | H::Freezing, W::Medium | W::High, M::Dry) => {
            actions.delta_crew(-2);
            actions.delta_items(Item::Gold, 300);
            actions.delta_food(50);
            actions.add_dialogue(captain!("We've lost two crew members to Atlantean traps, but we've acquired a fortune in gold and a supply of their miraculous preserved food!"));
        }
        (H::Blistering, W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-3);
            actions.delta_health(-25);
            actions.delta_items(Item::Gold, 200);
            actions.add_dialogue(captain!("Atlantis is sinking again! We barely escaped with our lives, losing three crew members and taking heavy damage. But we managed to grab some treasures on our way out!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 150);
            actions.delta_health(15);
            actions.delta_food(25);
            actions.add_dialogue(captain!("We explored cautiously and came away with a modest haul of Atlantean artifacts. Our ship and supplies have been bolstered by their advanced technology."));
        }
    }
}

fn negotiate_with_atlanteans(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(3);
        actions.delta_health(40);
        actions.delta_items(Item::Gold, 400);
        actions.add_dialogue(captain!("The Monkey's Paw glowed brightly, impressing the Atlanteans! They've gifted us with advanced technology, three of their own as new crew members, and a king's ransom in gold!"));
    } else {
        actions.delta_food(30);
        actions.delta_health(20);
        actions.delta_items(Item::Gold, 100);
        actions.add_dialogue(captain!("The Atlanteans were wary but ultimately friendly. They've provided us with some of their advanced food and medicine, along with a small gift of gold as a gesture of goodwill."));
    }
}

fn observe_from_distance(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We kept our distance and observed the magnificent city as it rose and sank back into the sea. While we didn't gain any material wealth, the crew seems invigorated by the once-in-a-lifetime spectacle."));
}

pub fn the_atlantis_resurfacing_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! I... I don't believe my eyes! A city is rising from the sea!"))
        .line(captain!("What madness is this? Speak clearly, man!"))
        .line(crew2!("It's true, sir! Golden spires, crystal domes... it can only be the lost city of Atlantis!"))
        .line(crew3!("Atlantis! The legends say it holds technology and riches beyond imagination. But also great dangers for the unprepared."))
        .line(captain!("This is a once in a lifetime opportunity, crew. But it could also be our doom. What shall we do?"))
        .choice("Explore Atlantis", explore_atlantis)
        .conditional_choice("Negotiate", negotiate_with_atlanteans, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Observe", observe_from_distance)
        .hint("Squawk! The greatest treasures often come with the greatest risks!")
}