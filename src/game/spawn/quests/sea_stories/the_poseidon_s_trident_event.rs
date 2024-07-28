use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn attempt_to_claim_trident(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable | H::Warm, W::Low | W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_health(10);
            actions.delta_crew(2);
            actions.add_dialogue(captain!("By Neptune's beard, we've done it! The Trident is ours! Our ship feels invincible, the seas bend to our will, and we've even gained some merperson allies!"));
        }
        (H::Chilly | H::Freezing, W::Medium | W::High, _) => {
            actions.delta_crew(-2);
            actions.delta_health(30);
            actions.delta_food(25);
            actions.add_dialogue(captain!("We couldn't claim the Trident, but Poseidon seemed impressed by our effort. He's blessed our ship with some of his power and provided us with an abundance of fish. Sadly, we lost two crew members to the sea."));
        }
        (H::Blistering, W::High | W::GaleForce, M::Humid) => {
            actions.delta_crew(-3);
            actions.delta_health(-10);
            actions.add_dialogue(captain!("Poseidon's wrath is terrible! We've lost three crew members, and our ship is badly damaged. The Trident has disappeared back into the depths."));
        }
        _ => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_health(20);
            actions.delta_food(25);
            actions.add_dialogue(captain!("We couldn't reach the Trident, but we found a sunken treasure on the way! Our ship feels somewhat empowered, and we've gathered plenty of fish."));
        }
    }
}

fn negotiate_with_poseidon(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(20);
        actions.delta_items(Item::Gold, 300);
        actions.add_dialogue(captain!("The Monkey's Paw glowed with an otherworldly light, catching Poseidon's attention! He's granted us some of his power, two merperson crew members, and a chest of divine gold coins!"));
    } else {
        actions.delta_food(75);
        actions.delta_health(25);
        actions.delta_items(Item::Gold, 100);
        actions.add_dialogue(captain!("Poseidon appreciated our respectful approach. While he kept his Trident, he's blessed our journey with favorable winds, bountiful fish, and a small tribute of gold."));
    }
}

fn observe_from_afar(actions: &mut StoryActions) {
    actions.delta_food(25);
    actions.delta_health(10);
    actions.add_dialogue(captain!("We wisely kept our distance. The display of godly power was awe-inspiring! While we gained no treasures, the crew's morale is soaring, and we found some good fishing spots."));
}

pub fn the_poseidon_s_trident_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The sea's churning something fierce, and there's a glow coming from the depths!"))
        .line(captain!("Steady as she goes! What in the seven seas is happening?"))
        .line(crew2!("It's... it's rising from the water, sir! A massive, glowing trident!"))
        .line(crew3!("It's Poseidon's Trident, Cap'n! Legend says it grants control over the seas themselves!"))
        .line(captain!("Neptune's beard! This could change everything... or doom us all. What's our move, crew?"))
        .choice("Claim Trident", attempt_to_claim_trident)
        .conditional_choice("Negotiate", negotiate_with_poseidon, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Observe", observe_from_afar)
        .hint("Squawk! The gods' favors can turn the tides of fortune!")
}