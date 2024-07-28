use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn navigate_maelstrom(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 200);
            actions.delta_health(15);
            actions.delta_crew(2);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("Inconceivable! We've navigated the Maelstrom of Realities! Our ship has transcended normal physics, we've acquired impossible treasures, and our crew now includes beings from other dimensions!"));
        }
        (H::Warm | H::Chilly, W::Low | W::High, M::Dry | M::Humid) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(10);
            actions.delta_crew(-3);
            actions.delta_items(Item::Cannon, 2);
            actions.add_dialogue(captain!("We've survived the Maelstrom, but at a cost. Three crew members were lost to other realities, but our ship now bends the laws of nature, and we've acquired artifacts that defy explanation."));
        }
        (H::Blistering | H::Freezing, W::GaleForce, _) => {
            actions.delta_crew(-6);
            actions.delta_health(-20);
            actions.delta_items(Item::Gold, 50);
            actions.add_dialogue(captain!("The Maelstrom nearly tore us apart! We've lost six crew members to alternate realities, and our ship is critically damaged. But the few artifacts we managed to grab are beyond priceless."));
        }
        _ => {
            actions.delta_items(Item::Gold, 80);
            actions.delta_health(10);
            actions.delta_food(25);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("We navigated the edge of the Maelstrom cautiously. Our ship has been imbued with some reality-altering properties, and we've acquired a haul of inter-dimensional treasures and provisions."));
        }
    }
}

fn harness_maelstrom_energy(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(10);
        actions.delta_items(Item::Gold, 100);
        actions.delta_items(Item::Cannon, 1);
        actions.add_dialogue(captain!("The Monkey's Paw resonated with the Maelstrom's energies! We've harnessed the power of multiple realities, recruited crew from parallel worlds, and our ship can now manipulate the fabric of space-time itself!"));
    } else {
        actions.delta_food(10);
        actions.delta_health(-10);
        actions.delta_items(Item::Gold, 40);
        actions.delta_crew(-2);
        actions.add_dialogue(captain!("Our attempt to harness the Maelstrom's energy was partially successful. We've gained some control over reality-warping powers, but two crew members were twisted into unrecognizable forms. The treasures we've acquired, however, are beyond imagination."));
    }
}

fn avoid_maelstrom(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_health(-5);
    actions.delta_items(Item::Gold, 30);
    actions.add_dialogue(captain!("We steered clear of the Maelstrom with all haste. The effort strained our ship and depleted our supplies, but we managed to collect some reality-warped debris floating in its wake. The crew is shaken but grateful to be alive and sane."));
}

pub fn the_maelstrom_of_realities_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! There's a... a hole in the ocean ahead! And I swear I can see other worlds in it!"))
        .line(captain!("What madness is this? Speak clearly, man!"))
        .line(crew2!("It's a massive whirlpool, sir, but it's... wrong! The water's all colors at once, and I can see impossible things in its depths!"))
        .line(crew3!("It's the Maelstrom of Realities, Cap'n! A nexus where all possible worlds collide! It's said to hold wonders and horrors beyond comprehension!"))
        .line(captain!("The Maelstrom of Realities... I thought it only a legend. This could rewrite the very nature of our existence. What's our move, crew?"))
        .choice("Navigate", navigate_maelstrom)
        .conditional_choice("Harness Energy", harness_maelstrom_energy, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Avoid", avoid_maelstrom)
        .hint("Squawk! Reality is but a suggestion to those brave enough to challenge it!")
}