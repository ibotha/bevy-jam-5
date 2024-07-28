use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn accept_race_challenge(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::High | W::GaleForce, M::Dry | M::Comfortable) => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_health(20);
            actions.add_dialogue(captain!("We did it! We outran the Flying Dutchman! They honored the bet, showering us with gold, and our ship seems faster than ever!"));
        }
        (W::Medium, _) => {
            actions.delta_items(Item::Gold, 150);
            actions.delta_crew(-1);
            actions.delta_health(10);
            actions.add_dialogue(captain!("A close race! We won by a hair's breadth, but lost a crew member to the Dutchman's crew. Still, we got some gold and our ship feels a bit swifter."));
        }
        (W::Low | W::None, M::Humid) => {
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("We lost the race and the Dutchman claimed two of our crew! Our ship took some spectral damage in the process. A harsh lesson indeed."));
        }
        _ => {
            actions.delta_food(20);
            actions.delta_health(5);
            actions.add_dialogue(captain!("It was a tie! The Dutchman's captain was impressed by our skill and gifted us with some of their ethereal provisions. Our ship feels somewhat rejuvenated from the experience."));
        }
    }
}

fn engage_in_riddle_contest(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_items(Item::Gold, 100);
        actions.delta_crew(1);
        actions.delta_health(15);
        actions.add_dialogue(captain!("The Monkey's Paw glowed during the contest, guiding our answers! We won handily, earning gold, a new spectral crew member, and some mystical ship enhancements!"));
    } else {
        actions.delta_items(Item::Gold, -100);
        actions.delta_food(15);
        actions.add_dialogue(captain!("The riddles were fiendishly difficult. We lost some gold in the contest, but the Dutchman's captain admired our effort and gave us some of their preserved rations as a consolation."));
    }
}

fn decline_challenge(actions: &mut StoryActions) {
    actions.delta_food(-10);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We politely declined the challenge. The Dutchman's crew seemed disappointed but respected our decision. They disappeared into the mist, leaving behind an eerie calm."));
}

pub fn the_flying_dutchman_challenge_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! A ghost ship has appeared off our starboard bow!"))
        .line(captain!("Steady, crew. What manner of ghost ship?"))
        .line(crew2!("It's... it's the Flying Dutchman, sir! Clear as day, but spectral and surrounded by an eerie glow."))
        .line(crew3!("They're... they're hailing us, Cap'n! The Dutchman's captain is challenging us to a contest!"))
        .line(captain!("A challenge from the Flying Dutchman itself? This could be interesting... or deadly. What say you, crew?"))
        .choice("Accept Race", accept_race_challenge)
        .conditional_choice("Riddle Contest", engage_in_riddle_contest, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Decline", decline_challenge)
        .hint("Squawk! Even the undead enjoy a good competition now and then!")
}