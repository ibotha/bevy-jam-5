use super::{
    constants::{CAPTAIN, CREW_MEMBER},
    dialogue::Dialogue,
    Certainty, DayEvent, Environment, FollowingEvent, StoryActions,
};

fn embark(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: visit_shady_cove(),
        certainty: Certainty::Certain,
        distance: 20,
        environment: Environment::Sea,
    });
    actions.add_dialogue(Dialogue::new(CAPTAIN, &["And so we go to shady cove!"]));
    actions.change_environment(Environment::Sea);
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

fn explore_shady_cove(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: shady_cove_treasure(),
        certainty: Certainty::Certain,
        distance: 10,
        environment: Environment::Island,
    });
    actions.add_dialogue(Dialogue::new(CAPTAIN, &["Adventure awaits!"]));
    actions.change_environment(Environment::Island);
}

pub fn visit_shady_cove() -> DayEvent {
    DayEvent::new(
        &[Dialogue::new(CAPTAIN, &["Oh my, is that shady cove?"])],
        &[("Explore", explore_shady_cove)],
    )
}

fn take_shady_cove_treasure(journey: &mut StoryActions) {
    journey.delta_crew(-3);
    journey.delta_max_crew(-3);
    journey.delta_health(-3);
    journey.delta_max_health(-3);
    journey.add_event(FollowingEvent {
        event: shady_cove_treasure(),
        certainty: Certainty::Certain,
        distance: 10,
        environment: Environment::Island,
    });
    journey.add_dialogue(Dialogue::new(CAPTAIN, &["Adventure awaits!"]));
    journey.change_environment(Environment::Island);
}

pub fn shady_cove_treasure() -> DayEvent {
    DayEvent::new(
        &[Dialogue::new(
            CAPTAIN,
            &["Arghh! Thats the treasure of shady cove!"],
        )],
        &[("Take", take_shady_cove_treasure)],
    )
}
