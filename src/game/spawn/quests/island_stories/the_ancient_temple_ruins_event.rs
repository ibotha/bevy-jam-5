use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn explore_temple(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_health(5);
            actions.add_dialogue(captain!("By the stars! We've found a hidden chamber filled with ancient treasure. The dry weather made exploration easy, and we're leaving with quite a haul!"));
        }
        (H::Warm, W::Medium, M::Humid) => {
            actions.delta_items(Item::Gold, 25);
            actions.delta_crew(-1);
            actions.delta_health(-5);
            actions.add_dialogue(captain!("We found some valuable artifacts, but the humid air made the ruins unstable. We lost a crew member to a collapsing passageway, and the ship took some damage during our hasty escape."));
        }
        (H::Blistering, _, _) | (_, W::GaleForce, _) => {
            actions.delta_health(-15);
            actions.delta_crew(-2);
            actions.add_dialogue(captain!("Curse these treacherous ruins! The extreme conditions triggered traps and collapses. We lost two good men and barely made it out with our lives. This temple is better left unexplored."));
        }
        _ => {
            actions.delta_items(Item::Gold, 15);
            actions.delta_food(-10);
            actions.add_dialogue(captain!("We managed to find a modest stash of gold coins, but the exploration took longer than expected. Our food supplies have dwindled a bit."));
        }
    }
}

fn decipher_inscriptions(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -5);
    actions.delta_items(Item::Cannon, 1);
    actions.add_dialogue(captain!("We spent some coin on supplies to properly document the inscriptions. Turns out they contained the designs for an advanced cannon! Our ship's now equipped with this powerful new weapon."));
}

fn leave_ruins(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 5);
    actions.delta_health(10);
    actions.add_dialogue(captain!("We decided not to risk the dangers of the temple. As we were leaving, we found a small offering bowl with some gold coins. The crew's relieved we didn't venture inside, and their spirits are high."));
}

pub fn the_ancient_temple_ruins_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! We've discovered ancient ruins on this island!"))
        .line(captain!("Fascinating... What can you make of them?"))
        .line(crew2!("Looks like some sort of temple, Cap'n. Could be traps inside, but might also be treasure!"))
        .line(crew3!("There are strange inscriptions on the walls outside. Could be valuable information if we could decipher them."))
        .line(captain!("This could be a significant find, crew. But it's not without risks. What's our move?"))
        .choice("Explore Temple", explore_temple)
        .conditional_choice("Decipher Inscriptions", decipher_inscriptions, actions.get_item(Item::Gold) >= 50)
        //.choice("Leave Ruins", leave_ruins)
        .hint("Squawk! Ancient treasures often come with ancient curses!")
}

