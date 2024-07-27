use super::port_stories_base;
use crate::game::spawn::quests::prelude::*;
use crate::game::spawn::quests::sea_events::set_next_port;
use crate::game::spawn::quests::sea_stories::the_bounty_hunters_event;

fn go_to_the_carnival(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind: _,
        moisture,
    } = actions.weather();

    match moisture {
        M::Dry => {
            actions.delta_crew(1);
            actions.delta_food(5);
        }
        M::Comfortable => {
            actions.delta_crew(2);
            actions.delta_food(20);
        }
        M::Humid => {
            actions.delta_crew(-1);
        }
    }
}

fn steal_from_the_armory(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind: _,
        moisture,
    } = actions.weather();

    match moisture {
        M::Dry => {
            actions.delta_crew(-1);
            actions.add_event(FollowingEvent {
                environment: Environment::Sea(actions.get_current_sea()),
                delay: Delay::Distance(18),
                event: the_bounty_hunters_event,
                certainty: Certainty::Possible(2),
            });
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(crew1!("We lost one captain."));
            actions.add_dialogue(captain!(
                "Something tells me we aren't welcome here anymore."
            ));
            set_next_port(actions, 20);
        }
        M::Comfortable => {
            actions.delta_food(20);
            actions.delta_items(Item::Gold, 50);
            actions.delta_items(Item::Cannon, 2)
        }
        M::Humid => {
            actions.delta_crew(-2);
            actions.delta_items(Item::Gold, -100);
            actions.add_event(FollowingEvent {
                environment: Environment::Sea(actions.get_current_sea()),
                delay: Delay::Distance(18),
                event: the_bounty_hunters_event,
                certainty: Certainty::Possible(2),
            });
            set_next_port(actions, 20);
            actions.add_dialogue(captain!(
                "All the guards were at  there post!",
                "We aren't getting those men or supplies back..."
            ));
            actions.add_dialogue(captain!(
                "Something tells me we aren't welcome here anymore."
            ));
        }
    }
}

pub fn the_carnival_in_the_city_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Captian! Looks like there is a party in the city."))
        .choice("Party!", go_to_the_carnival)
        .choice("Steal", steal_from_the_armory)
        .hint("Squawk! RaIny FESTivals are NO FUN!")
}

