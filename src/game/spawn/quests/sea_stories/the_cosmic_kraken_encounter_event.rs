use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn engage_cosmic_kraken(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 2500);
            actions.delta_health(200);
            actions.delta_crew(10);
            actions.delta_items(Item::Cannon, 15);
            actions.add_dialogue(captain!("By all the gods! We've subdued the Cosmic Kraken! Our ship is infused with cosmic energy, our holds burst with interdimensional treasures, and beings from across the multiverse have joined our crew!"));
        }
        (H::Warm | H::Chilly, W::Low | W::High, M::Dry | M::Humid) => {
            actions.delta_items(Item::Gold, 1200);
            actions.delta_health(-50);
            actions.delta_crew(-4);
            actions.delta_items(Item::Cannon, 8);
            actions.add_dialogue(captain!("We've survived the Cosmic Kraken's onslaught, but at a terrible cost. Four crew members were lost to other dimensions, our ship is damaged, but we've acquired cosmic artifacts and our cannons now fire interdimensional energy!"));
        }
        (H::Blistering | H::Freezing, W::GaleForce, _) => {
            actions.delta_crew(-8);
            actions.delta_health(-100);
            actions.delta_items(Item::Gold, 600);
            actions.add_dialogue(captain!("The Cosmic Kraken nearly destroyed us! We've lost eight crew members to the void, our ship is barely holding together, but we managed to snatch a few priceless cosmic relics as we fled."));
        }
        _ => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_health(50);
            actions.delta_food(300);
            actions.delta_items(Item::Cannon, 5);
            actions.add_dialogue(captain!("We engaged the Cosmic Kraken cautiously. While we couldn't defeat it, we've collected some of its cosmic essence. Our ship can now briefly phase through reality, and we've acquired some truly alien treasures and technology."));
        }
    }
}

fn attempt_cosmic_communion(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(8);
        actions.delta_health(150);
        actions.delta_items(Item::Gold, 2000);
        actions.delta_items(Item::Cannon, 12);
        actions.add_dialogue(captain!("The Monkey's Paw resonated with the Cosmic Kraken's unfathomable mind! We've achieved a symbiosis with the creature. Our ship can now traverse dimensions, we've gained cosmic awareness, and emissaries from countless realities have joined our crew!"));
    } else {
        actions.delta_food(200);
        actions.delta_health(-60);
        actions.delta_items(Item::Gold, 500);
        actions.delta_crew(-3);
        actions.add_dialogue(captain!("Our attempt at cosmic communion was overwhelming. We've gained glimpses of cosmic truths and some interdimensional artifacts, but three crew members' minds were lost to the vastness of the multiverse."));
    }
}

fn flee_cosmic_kraken(actions: &mut StoryActions) {
    actions.delta_food(-150);
    actions.delta_health(-30);
    actions.delta_items(Item::Gold, 400);
    actions.add_dialogue(captain!("We fled from the Cosmic Kraken with all haste. The effort severely strained our ship and depleted our supplies, but we managed to collect some cosmic debris in our wake. The crew is terrified but grateful to still exist in this reality."));
}

pub fn the_cosmic_kraken_encounter_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The... the sky is breaking! And something's coming through!"))
        .line(captain!("Breaking? What in the name of all that's holy do you mean?"))
        .line(crew2!("It's... it's massive, sir! Tentacles the size of continents, reaching through tears in reality itself!"))
        .line(crew3!("It's the Cosmic Kraken, Cap'n! The interdimensional leviathan that swims through the very fabric of existence!"))
        .line(captain!("The Cosmic Kraken... I never thought I'd witness such a thing. This encounter could redefine our very understanding of reality. What's our course, crew?"))
        .choice("Engage", engage_cosmic_kraken)
        .conditional_choice("Commune", attempt_cosmic_communion, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Flee", flee_cosmic_kraken)
        .hint("Squawk! Some creatures transcend the boundaries of our world, and our imagination!")
}