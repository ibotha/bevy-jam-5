use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn investigate_eruption(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture: _,
    } = actions.weather();

    match (heat, wind) {
        (H::Blistering | H::Warm, W::Low | W::Medium) => {
            actions.delta_items(Item::Gold, 20);
            actions.delta_health(-10);
            actions.delta_food(15);
            actions.add_dialogue(captain!("Incredible! The eruption brought up ancient artifacts and rare fish. We've collected some valuable items, though our ship took minor damage from the heat."));
        }
        (H::Comfortable, W::Medium | W::High) => {
            actions.delta_items(Item::Cannon, 1);
            actions.delta_health(15);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("We found a peculiar metallic object in the eruption debris - it seems to be an advanced cannon! Sadly, we lost a crew member to an unexpected steam burst."));
        }
        (H::Chilly | H::Freezing, W::High | W::GaleForce) => {
            actions.delta_crew(-2);
            actions.delta_health(-15);
            actions.delta_food(20);
            actions.add_dialogue(captain!("The combination of cold air and hot water created treacherous conditions. We lost two crew members and took significant damage, but collected an abundance of stunned rare fish."));
        }
        _ => {
            actions.delta_items(Item::Gold, 15);
            actions.delta_food(15);
            actions.add_dialogue(captain!("We cautiously collected some minerals and fish brought up by the eruption. Nothing extraordinary, but it's a welcome addition to our stores."));
        }
    }
}

fn harness_thermal_energy(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_health(25);
        actions.delta_food(25);
        actions.delta_items(Item::Gold, 50);
        actions.add_dialogue(captain!("The Monkey's Paw glowed as we approached the volcano! We've somehow harnessed its energy, greatly strengthening our ship. We also collected valuable minerals and a bounty of cooked fish!"));
    } else {
        actions.delta_health(-5);
        actions.delta_food(30);
        actions.add_dialogue(captain!("We managed to use the volcano's heat to cook a large amount of fish, but the attempt strained our ship a bit. Still, our food stores are now plentiful."));
    }
}

fn retreat_to_safe_waters(actions: &mut StoryActions) {
    actions.delta_food(-5);
    actions.delta_health(5);
    actions.add_dialogue(captain!("We wisely retreated to safer waters. We used a bit more supplies in the detour, but the crew is relieved and our ship avoided any potential damage."));
}

pub fn the_underwater_volcano_eruption_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The water's starting to bubble and steam! Something's not right!"))
        .line(captain!("Steady, crew! What's causing this disturbance?"))
        .line(crew2!("It looks like an underwater volcano eruption, sir! The sea's getting choppy and there's debris surfacing!"))
        .line(crew3!("I've heard tales of rare treasures brought up by such eruptions, Cap'n. But it could be dangerous to get too close."))
        .line(captain!("An underwater volcano, eh? This could be interesting... or deadly. What's our move, crew?"))
        .choice("Investigate", investigate_eruption)
        .conditional_choice("Harness Energy", harness_thermal_energy, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Retreat", retreat_to_safe_waters)
        .hint("Squawk! Nature's fury can bring both peril and opportunity!")
}