use mysterious_island::sighted_mysterious_island;
use royal_navy_base::sighted_navy_base;
use trinket_seller::sighted_trinket_seller;

use super::prelude::*;
pub mod mysterious_island;
pub mod royal_navy_base;
pub mod trinket_seller;

fn trinket_seller(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(Sea::Northern));
    actions.add_event(FollowingEvent {
        event: sighted_trinket_seller,
        certainty: Certainty::Certain,
        environment: Environment::Sea(Sea::Northern),
        delay: Delay::Distance(16),
    });
}

fn navy_base(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(Sea::Northern));
    actions.add_event(FollowingEvent {
        event: sighted_navy_base,
        certainty: Certainty::Certain,
        environment: Environment::Sea(Sea::Northern),
        delay: Delay::Distance(10),
    });
}

fn mysterious_island(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(Sea::Northern));
    actions.add_event(FollowingEvent {
        event: sighted_mysterious_island,
        certainty: Certainty::Certain,
        environment: Environment::Sea(Sea::Northern),
        delay: Delay::Distance(16),
    });
}

pub fn set_course_northern_sea(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .choice("Trinket Seller", trinket_seller)
        .conditional_choice(
            "Navy Base",
            navy_base,
            actions.get_item(Item::SirensCoveMap) == 0,
        )
        .conditional_choice(
            "Siren's Cove",
            navy_base,
            actions.get_item(Item::SirensCoveMap) > 0,
        )
        .conditional_choice(
            "Myst Island",
            mysterious_island,
            actions.get_item(Item::SirensScale) == 0,
        )
}
