use rand::RngCore;

use super::{
    prelude::*,
    sea_events::{bounty_hunters, sail},
};

fn embark(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea);
    sail(actions);
}

fn recruit(actions: &mut StoryActions) {
    actions.delta_crew(1);
}

fn resupply(actions: &mut StoryActions) {
    actions.delta_food(10);
}
fn repair(actions: &mut StoryActions) {
    actions.delta_health(10);
}

fn a_day_at_port() -> DayEvent {
    DayEvent::new()
        .line(
            Dialogue::new(CAPTAIN)
                .para("We are still at the blasted port!")
                .para("Is the weather right for us to embark?"),
        )
        .choice("Embark", embark)
        .choice("Recruit", recruit)
        .choice("Resupply", resupply)
        .choice("Repair", repair)
}

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
                environment: super::Environment::Sea,
                distance: 18,
                event: bounty_hunters(),
                certainty: super::Certainty::Possible(2),
            });
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

fn a_carnival_in_the_city() -> DayEvent {
    DayEvent::new()
        .line(Dialogue::new(CREW1).para("Captian! Looks like there is a party in the city."))
        .choice("Party!", go_to_the_carnival)
        .choice("Steal", steal_from_the_armory)
        .hint("Squawk! RaIny FESTivals are NO FUN!")
}

pub(super) fn select_random_port_event(rng: &mut impl RngCore) -> DayEvent {
    weighted_random(
        Some(rng),
        &[(a_carnival_in_the_city(), 1), (a_day_at_port(), 14)],
    )
    .clone()
}
