use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn enter_portal(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1500);
            actions.delta_health(120);
            actions.delta_crew(6);
            actions.delta_items(Item::Cannon, 8);
            actions.add_dialogue(captain!("Unbelievable! We've explored Atlantis and returned! Our ship is infused with ancient technology, our holds overflow with Atlantean treasures, and we've recruited Atlantean scholars to our crew!"));
        }
        (H::Warm | H::Chilly, W::Low | W::High, M::Dry | M::Humid) => {
            actions.delta_items(Item::Gold, 800);
            actions.delta_health(60);
            actions.delta_crew(-2);
            actions.delta_items(Item::Cannon, 5);
            actions.add_dialogue(captain!("We've returned from Atlantis, but at a cost. Two crew members were lost to the city's defenses, but we've acquired incredible Atlantean artifacts, advanced weaponry, and our ship has been greatly enhanced."));
        }
        (H::Blistering | H::Freezing, W::GaleForce, _) => {
            actions.delta_crew(-4);
            actions.delta_health(-60);
            actions.delta_items(Item::Gold, 400);
            actions.add_dialogue(captain!("The portal was unstable in these conditions! We barely escaped Atlantis with our lives. We lost four crew members and the ship is badly damaged, but we managed to grab some Atlantean treasures before fleeing."));
        }
        _ => {
            actions.delta_items(Item::Gold, 600);
            actions.delta_health(40);
            actions.delta_food(200);
            actions.delta_items(Item::Cannon, 3);
            actions.add_dialogue(captain!("We explored Atlantis cautiously. While we couldn't stay long, we've returned with valuable artifacts, advanced Atlantean cannons, and our ship has been partially upgraded with their technology."));
        }
    }
}

fn study_portal(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(4);
        actions.delta_health(100);
        actions.delta_items(Item::Gold, 1000);
        actions.delta_items(Item::Cannon, 6);
        actions.add_dialogue(captain!("The Monkey's Paw resonated with the Atlantean portal! We've gained incredible insights into Atlantean science, recruited Atlantean emissaries, and our ship now harnesses the power of Atlantis itself!"));
    } else {
        actions.delta_food(100);
        actions.delta_health(-20);
        actions.delta_items(Item::Gold, 300);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("Our attempts to study the portal yielded some results, but the energies were overwhelming. We've gained some Atlantean knowledge and artifacts, but lost one crew member to the portal's unpredictable energies."));
    }
}

fn seal_the_portal(actions: &mut StoryActions) {
    actions.delta_food(50);
    actions.delta_health(20);
    actions.delta_items(Item::Gold, 200);
    actions.add_dialogue(captain!("We decided to seal the portal for the safety of all. The Atlanteans, grateful for our discretion, rewarded us with some of their technology and treasures. The crew feels proud of our responsible decision."));
}

pub fn the_atlantean_portal_discovery_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! You won't believe this, but there's a shimmering... doorway in the water!"))
        .line(captain!("A doorway? In the middle of the ocean? Explain yourself!"))
        .line(crew2!("It's true, sir! It's like a window to another world. I can see spires and structures unlike anything I've ever seen!"))
        .line(crew3!("By Neptune's beard, it must be a portal to Atlantis! The lost city of legend!"))
        .line(captain!("Atlantis... This could be the discovery of the millennium. But it could also spell our doom if we're not careful. What shall we do, crew?"))
        .choice("Enter Portal", enter_portal)
        .conditional_choice("Study Portal", study_portal, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Seal Portal", seal_the_portal)
        .hint("Squawk! The greatest treasures often lie beyond the veil of our world!")
}