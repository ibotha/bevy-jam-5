use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn challenge_kraken_god(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::GaleForce, M::Humid) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_items(Item::Cannon, 20);
            actions.delta_health(25);
            actions.delta_crew(5);
            actions.add_dialogue(captain!("BY ALL THE GODS AND DEVILS! We've done the impossible! In the heart of the maelstrom, we defeated the Kraken God! Our ship is now a living legend, morphing at will. Our cannons fire bolts of lightning, and the crew... the crew are demigods walking among mortals. The oceans themselves part at our command!"));
        }
        (W::High, M::Comfortable) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_items(Item::Cannon, 10);
            actions.delta_health(20);
            actions.delta_crew(5);
            actions.add_dialogue(captain!("Victory, but at what cost? We've bested the Kraken God, but the battle has changed us forever. Our ship now phases through reality, our cannons shoot tentacles that pull enemy ships asunder, and half the crew have grown gills. The spoils of war are beyond imagination, but are we even human anymore?"));
        }
        (W::Low | W::None, _) => {
            actions.delta_crew(-3);
            actions.delta_health(-20);
            actions.delta_items(Item::Gold, 40);
            actions.add_dialogue(captain!("Disaster beyond comprehension! The Kraken God toyed with us in the calm seas. We've lost scores of men to its tentacles, and the ship is barely more than driftwood. Yet... we live, and we've claimed a piece of its divine treasure. If we survive, we'll be haunted by this day forever."));
        }
        _ => {
            actions.delta_items(Item::Gold, 20);
            actions.delta_items(Item::Cannon, 1);
            actions.delta_health(10);
            actions.delta_crew(-5);
            actions.add_dialogue(captain!("A stalemate with a god... Who would have thought it possible? We've weathered the storm of tentacles and divine wrath, losing good men but gaining riches beyond measure. Our ship now bears the mark of the Kraken, striking fear into all who see us. The legend of this day will live forever!"));
        }
    }
}

fn bargain_with_kraken_god(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 100 {
        actions.delta_items(Item::Gold, -10);
        actions.delta_items(Item::Cannon, 1);
        actions.delta_health(40);
        actions.delta_crew(4);
        actions.add_dialogue(captain!("Who would have thought a god could be reasoned with? We've struck a deal with the Kraken God! In exchange for our worldly treasure, it has blessed us with divine power. Our ship can now dive beneath the waves, our crew can breathe water, and our cannons... By the stars, our cannons can summon whirlpools! The age of the Kraken Pirates begins now!"));
    } else {
        actions.delta_health(-15);
        actions.delta_crew(-2);
        actions.delta_items(Item::Gold, 50);
        actions.add_dialogue(captain!("Folly! Our offering was an insult to the Kraken God! It has cursed us to roam the seas forever, neither living nor dead. Our ship is now a ghost of its former self, crewed by specters. Yet... in its twisted humor, the god left us with treasure. A cruel joke for those who can no longer enjoy mortal pleasures."));
    }
}

fn flee_kraken_god(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 50);
    actions.delta_items(Item::Cannon, 1);
    actions.delta_health(25);
    actions.delta_crew(2);
    actions.add_dialogue(captain!("Call me a coward, but we live to sail another day! As we fled, the Kraken God's laughter shook the very sky. It rained gold and cannons as if mocking our retreat. Yet... I feel its eye upon us still. We've gained much, but I fear we've drawn the attention of forces beyond our understanding. Our legend grows, but at what cost to our souls?"));
}

pub fn the_awakening_of_the_kraken_god_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("C-C-CAPTAIN! The sea... it's alive! A mountain of tentacles rises from the depths!"))
        .line(captain!("Impossible... Could it be? The legendary Kraken God awakens?"))
        .line(crew2!("It's real, Cap'n! I see a crown of shipwrecks upon its head, and eyes deeper than the abyss!"))
        .line(crew3!("They say it guards the ultimate treasure... and can grant power over the very seas themselves!"))
        .line(captain!("This is it, lads. The moment every pirate dreams of but never lives to see. We stand at the crossroads of legend. What say you? Shall we risk everything for glory eternal?"))
        .choice("Challenge God", challenge_kraken_god)
        .conditional_choice("Bargain", bargain_with_kraken_god, actions.get_item(Item::Gold) >= 10000)
        //.choice("Flee", flee_kraken_god)
        .hint("Squawk! Some legends are born, others are claimed... and some are drowned in the depths!")
}

