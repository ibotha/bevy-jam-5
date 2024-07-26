use std::collections::VecDeque;

use crate::game::spawn::{journey::Ship, weather::DayWeather};

use super::{
    constants::{CAPTAIN, CREW_MEMBER},
    dialogue::Dialogue,
    Certainty, ChoiceResult, DayEvent, Environment, FollowingEvent,
};

fn embark(ship: Ship, weather: &DayWeather) -> ChoiceResult {
    ChoiceResult {
        ship,
        following_events: vec![FollowingEvent {
            event: visit_shady_cove(),
            certainty: Certainty::Certain,
            distance: 20,
            environment: Environment::Sea,
        }],
        dialogues: VecDeque::from_iter([Dialogue::new(CAPTAIN, &["And so we go to shady cove!"])]),
    }
}

pub fn embark_event() -> DayEvent {
    DayEvent::new(
        &[
            Dialogue::new(CAPTAIN, &["We are headed to get the trident!!!"]),
            Dialogue::new(CREW_MEMBER, &["What?!?!? The trident!!!"]),
            Dialogue::new(CAPTAIN, &["Yes! The trident"]),
        ],
        &[("Embark!", embark)],
    )
}

pub fn visit_shady_cove() -> DayEvent {
    DayEvent::new(
        &[Dialogue::new(CAPTAIN, &["Oh my, is that shady cove?"])],
        &[("Embark!", embark)],
    )
}
