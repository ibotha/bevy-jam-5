use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn confront_kraken(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::Low | W::None, M::Dry) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_items(Item::Cannon, 5);
            actions.delta_health(25);
            actions.delta_crew(5);
            actions.add_dialogue(captain!("By all the gods! We've done the impossible - we've defeated the Kraken! Its lair is a treasure trove beyond imagination. We've found ancient cannons of immense power, and the Kraken's essence has imbued our ship and crew with supernatural strength. We're legends now, mark my words!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_items(Item::Cannon, 2);
            actions.delta_crew(-3);
            actions.delta_health(-15);
            actions.add_dialogue(captain!("We've slain the beast, but at a terrible cost. Three brave souls lost to its tentacles, and the ship's barely holding together. Still, we've claimed a king's ransom in treasure and two of the Kraken's own cannons. The tales of this day will be told for generations!"));
        }
        (W::High | W::GaleForce, _) => {
            actions.delta_crew(-8);
            actions.delta_health(-30);
            actions.delta_items(Item::Gold, 100);
            actions.add_dialogue(captain!("Disaster upon disaster! The Kraken was invincible in this maelstrom. We've lost eight good men, and the ship's all but destroyed. We managed to break off one of its tentacles laden with gold as we fled, but the cost... oh, the cost is too high!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_items(Item::Cannon, 1);
            actions.delta_crew(-2);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("A pyrrhic victory if ever there was one. We've slain the Kraken and taken a hefty prize, including one of its mystical cannons. But we lost two of our finest, and the ship will need major repairs. The sea has exacted its price for our audacity."));
        }
    }
}

fn negotiate_with_kraken(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -100);
        actions.delta_items(Item::Cannon, 3);
        actions.delta_health(20);
        actions.delta_crew(3);
        actions.add_dialogue(captain!("I can't believe it worked! We offered the Kraken our gold as tribute, and it... spoke to us! It's granted us three of its mystical cannons and imbued our ship with incredible strength. It even returned some of our long-lost crew members! The sea will never be the same for us."));
    } else {
        actions.delta_health(-20);
        actions.delta_crew(-4);
        actions.add_dialogue(captain!("The Kraken was not impressed by our meager offerings. It spared us total destruction but took four of our crew as tribute. The ship's barely afloat, and we'll be having nightmares for years to come."));
    }
}

fn flee_kraken(actions: &mut StoryActions) {
    actions.delta_food(-50);
    actions.delta_health(-15);
    actions.delta_items(Item::Gold, 50);
    actions.delta_crew(-1);
    actions.add_dialogue(captain!("We turned tail and fled, pushing our ship beyond its limits. We lost one man overboard and the ship's taken a beating, but as we sailed away, a tentacle dropped a chest of gold on our deck. A parting gift... or a curse? Only time will tell."));
}

pub fn the_krakens_awakening_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("CAP'N! IT'S... IT'S THE KRAKEN! IT'S REAL!"))
        .line(captain!("Impossible... the Kraken is just a legend... isn't it?"))
        .line(crew2!("Legend's come to life, Cap'n! It's as big as a mountain and twice as angry!"))
        .line(crew3!("They say the Kraken guards treasures beyond imagination... if we survive, we'd be set for life!"))
        .line(captain!("This is it, lads. The moment that will define us for eternity. Do we face the legend, or run from it?"))
        .choice("Confront Kraken", confront_kraken)
        .conditional_choice("Negotiate", negotiate_with_kraken, actions.get_item(Item::Gold) >= 1000)
        .choice("Flee", flee_kraken)
        .hint("Squawk! Some legends are best left in storybooks... but others can make you a legend yourself!")
}