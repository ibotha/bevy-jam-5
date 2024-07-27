use monster_hunters::monster_hunters;
use mysterious_island::sighted_mysterious_island;
use royal_navy_base::sighted_navy_base;
use trinket_seller::sighted_trinket_seller;

use super::{prelude::*, sirens_cove::to_sirens_cove};
pub mod monster_hunters;
pub mod mysterious_island;
pub mod royal_navy_base;
pub mod trinket_seller;

fn trinket_seller(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(Sea::Northern));
    if actions.once_off("Monster Hunters") {
        actions.add_event(FollowingEvent {
            event: monster_hunters,
            certainty: Certainty::Possible(10),
            environment: Environment::Sea(Sea::Northern),
            delay: Delay::Distance(10),
        });
    }
    actions.add_event(FollowingEvent {
        event: sighted_trinket_seller,
        certainty: Certainty::Certain,
        environment: Environment::Sea(Sea::Northern),
        delay: Delay::Distance(20),
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
        .line(captain!(
            "The world is our oyster, we can go anywhere in the northern sea."
        ))
        .conditional_choice(
            "Trinket",
            trinket_seller,
            actions.get_environment() != Environment::Island(Island::TrinketSeller),
        )
        .conditional_choice(
            "Navy",
            navy_base,
            actions.get_item(Item::SirensCoveMap) == 0
                && actions.get_environment() != Environment::Port(Port::RoyalNavyBase),
        )
        .conditional_choice(
            "Sirens",
            to_sirens_cove,
            actions.get_item(Item::SirensCoveMap) > 0
                && actions.get_environment() != Environment::Port(Port::EdgeOfTheWorld),
        )
        .conditional_choice(
            "Island",
            mysterious_island,
            actions.get_item(Item::SirensScale) == 0
                && actions.get_environment() != Environment::Island(Island::MysteriousIsland),
        )
}
