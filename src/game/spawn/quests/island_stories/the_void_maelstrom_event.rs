use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn enter_maelstrom(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_items(Item::Cannon, 5);
            actions.delta_health(25);
            actions.delta_crew(15);
            actions.add_dialogue(captain!("Unfathomable! We've traversed the Void Maelstrom and emerged as masters of reality! Our ship now phases through dimensions, our cannons erase matter from existence, and our crew manipulates the fabric of space-time. We've brought back riches from realms beyond comprehension!"));
        }
        (H::Warm, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_items(Item::Cannon, 4);
            actions.delta_crew(-5);
            actions.delta_health(15);
            actions.add_dialogue(captain!("We've survived the Void Maelstrom, but at a terrible price. Five crew members were unmade by the cosmic forces, but those who remain have gained impossible powers. Our ship now sails through shadows, and our holds are filled with otherworldly treasures that defy description."));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-20);
            actions.delta_health(-15);
            actions.delta_items(Item::Gold, 200);
            actions.add_dialogue(captain!("Catastrophe beyond measure! The Maelstrom nearly destroyed us completely. Most of our crew has been scattered across infinite realities, and our ship is barely clinging to existence. We've returned with void-touched gold, but the horrors we've witnessed will haunt us for eternity."));
        }
        _ => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_items(Item::Cannon, 2);
            actions.delta_crew(10);
            actions.delta_health(10);
            actions.add_dialogue(captain!("We've gazed into the abyss and returned... changed. Our ship now casts no shadow, our new cannons fire bolts of pure nothingness, and half the crew has developed eldritch abilities. Our coffers overflow with impossible riches, but I fear we've lost something of our humanity in that swirling void."));
        }
    }
}

fn harness_void_energy(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 1 {
        actions.delta_items(Item::Cannon, -1);
        actions.delta_items(Item::Gold, 250);
        actions.delta_health(20);
        actions.delta_crew(5);
        actions.add_dialogue(captain!("Inconceivable success! By sacrificing our cannons to the Void, we've harnessed its unlimited power! Our ship now exists in all realities simultaneously, our crew has ascended beyond mortality, and we've amassed wealth from a thousand dimensions. We're no longer pirates - we're the masters of the multiverse!"));
    } else {
        actions.delta_health(-10);
        actions.delta_crew(-15);
        actions.delta_items(Item::Gold, 30);
        actions.add_dialogue(captain!("Disastrous miscalculation! Without enough cannons to stabilize the Void energies, we were nearly torn apart. We've lost many crew to the hungry darkness, and our ship is now partially out of phase with reality. We've gathered some void-warped gold, but at what cost to our very existence?"));
    }
}

fn skirt_maelstrom(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 400);
    actions.delta_items(Item::Cannon, 2);
    actions.delta_health(5);
    actions.delta_crew(5);
    actions.add_dialogue(captain!("We navigated the edges of the Void Maelstrom, and even that was almost too much! Reality flickered around us, depositing impossible treasures on our deck and whisking away parts of our ship. We've gained riches and strange new cannons, and a few beings from other dimensions have joined our crew. The universe will never look the same to us again."));
}

pub fn the_void_maelstrom_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("CAP'N! There's a... a hole in the ocean ahead! It's swirling with darkness and impossible colors!"))
        .line(captain!("Great Poseidon's beard... it's the legendary Void Maelstrom!"))
        .line(crew2!("I see things in the swirling darkness, Cap'n... other worlds, impossible riches, and horrors beyond sanity!"))
        .line(crew3!("They say our cannons could be used to harness the Void's power, Cap'n... if we dare risk it all!"))
        .line(captain!("This is it, lads - the ultimate test of our courage and sanity. Do we dare to challenge the very fabric of reality itself?"))
        .choice("Enter Maelstrom", enter_maelstrom)
        .conditional_choice("Harness Void", harness_void_energy, actions.get_item(Item::Cannon) >= 10)
        .choice("Skirt Maelstrom", skirt_maelstrom)
        .hint("Squawk! When you stare into the abyss, sometimes the abyss stares back... and gives you treasures!")
}