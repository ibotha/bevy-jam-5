use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn harness_celestial_power(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1000);
            actions.delta_health(100);
            actions.delta_crew(5);
            actions.delta_items(Item::Cannon, 5);
            actions.add_dialogue(captain!("By all the gods! We've harnessed the celestial power! Our ship is transformed, our coffers overflow with cosmic gold, and we've been joined by celestial beings! We're unstoppable!"));
        }
        (H::Warm | H::Chilly, W::Low | W::High, M::Dry | M::Humid) => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_health(50);
            actions.delta_crew(-1);
            actions.delta_items(Item::Cannon, 3);
            actions.add_dialogue(captain!("We've tapped into the celestial energy, but at a cost. One crew member was lost to the cosmos, but our ship is greatly enhanced, our cannons are infused with stellar power, and we've acquired a fortune in astral gold!"));
        }
        (H::Blistering | H::Freezing, W::GaleForce, _) => {
            actions.delta_crew(-3);
            actions.delta_health(-50);
            actions.delta_items(Item::Gold, 250);
            actions.add_dialogue(captain!("The celestial forces were too powerful to control! We've lost three crew members to the void and our ship is badly damaged. But we managed to siphon some cosmic energy and a bit of celestial gold before retreating."));
        }
        _ => {
            actions.delta_items(Item::Gold, 300);
            actions.delta_health(25);
            actions.delta_food(100);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("We cautiously interacted with the celestial energies. Our ship is somewhat enhanced, our supplies are multiplied, and we've acquired some cosmic gold and a star-forged cannon."));
        }
    }
}

fn commune_with_celestials(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(3);
        actions.delta_health(75);
        actions.delta_items(Item::Gold, 750);
        actions.delta_items(Item::Cannon, 3);
        actions.add_dialogue(captain!("The Monkey's Paw resonated with the celestial energies, allowing us to communicate directly with cosmic entities! They've blessed our ship with unimaginable power, gifted us celestial crew members, and bestowed upon us riches beyond mortal dreams!"));
    } else {
        actions.delta_food(50);
        actions.delta_health(20);
        actions.delta_items(Item::Gold, 200);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("We attempted to commune with the celestial forces. We gained some cosmic insights and blessings, but the experience overwhelmed one of our crew members, who vanished into the astral plane."));
    }
}

fn observe_the_phenomenon(actions: &mut StoryActions) {
    actions.delta_food(25);
    actions.delta_health(10);
    actions.delta_items(Item::Gold, 50);
    actions.add_dialogue(captain!("We kept our distance and observed the celestial alignment. The sight was awe-inspiring and has lifted the crew's spirits. Some residual cosmic energy has enhanced our supplies and left us with a small amount of celestial gold."));
}

pub fn the_celestial_alignment_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The stars... they're moving! The sky is alive!"))
        .line(captain!("What in the name of all that's holy is happening to the heavens?"))
        .line(crew2!("It's a celestial alignment, sir! The likes of which hasn't been seen in a thousand years!"))
        .line(crew3!("Legend says such an alignment can grant godlike power... or utterly destroy those who seek it, Cap'n."))
        .line(captain!("This could be the most monumental moment of our lives. The power of the cosmos is within our reach... but at what risk? What shall we do, crew?"))
        .choice("Harness Power", harness_celestial_power)
        .conditional_choice("Commune", commune_with_celestials, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Observe", observe_the_phenomenon)
        .hint("Squawk! The stars align for those bold enough to reach for them!")
}