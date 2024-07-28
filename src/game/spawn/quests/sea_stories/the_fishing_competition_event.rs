use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn participate_in_competition(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::None | W::Low, M::Comfortable) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_food(15);
            actions.add_dialogue(captain!("Perfect conditions for fishing! We won the competition and got a nice haul of fish to boot."));
        }
        (W::Medium, M::Dry | M::Comfortable) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_food(10);
            actions.add_dialogue(captain!("We didn't win, but we caught enough fish to make it worthwhile."));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_crew(-1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The weather turned foul during the competition. We lost a crew member overboard and had to cut our lines!"));
        }
        _ => {
            actions.delta_food(5);
            actions.add_dialogue(captain!("The competition was a bust, but we managed to catch a few fish for our stores."));
        }
    }
}

fn watch_competition(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -10);
    actions.add_dialogue(captain!("We enjoyed watching the competition from the docks. The crew's spirits are lifted, even if our purse is a bit lighter."));
}

fn ignore_competition(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 25);
    actions.delta_food(-5);
    actions.add_dialogue(captain!("We used the distraction of the competition to do some trading. Made a small profit, but missed out on restocking our food stores."));
}

pub fn the_fishing_competition_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's a big fishing competition happening in the harbor today!"))
        .line(captain!("A fishing competition, you say? Interesting..."))
        .line(crew2!("Aye, sir! First prize is 100 gold coins and the catch itself!"))
        .line(crew3!("It could be a good chance to fill our stores and have some fun, Cap'n."))
        .line(captain!("Hmm, what do you think we should do, crew?"))
        .choice("Participate", participate_in_competition)
        .choice("Watch", watch_competition)
        .choice("Ignore", ignore_competition)
        .hint("Squawk! A good catch can fill both belly and coin purse!")
}