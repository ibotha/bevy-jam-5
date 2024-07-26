use super::{
    constants::*, dialogue::Dialogue, treasure::Item, Certainty, DayEvent, Environment,
    FollowingEvent, StoryActions,
};

fn embark(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: visit_shady_cove(),
        certainty: Certainty::Certain,
        distance: 20,
        environment: Environment::Sea,
    });
    actions.add_dialogue(
        Dialogue::new(CAPTAIN)
            .para("And so we go to shady cove!")
            .clone(),
    );
    actions.change_environment(Environment::Sea);
}

pub fn embark_event() -> DayEvent {
    DayEvent::new()
        .line(Dialogue::new(CREW1).para("Ahoy sage! Have ye heard the news?"))
        .line(
            Dialogue::new(CREW1)
                .para("Word about port is there is a great treasure on the horizon."),
        )
        .line(
            Dialogue::new(CREW2)
                .para("Aye, I heard the same.")
                .para("Reckon we got a shot at it? Harhar..."),
        )
        .line(
            Dialogue::new(CAPTAIN)
                .para("Alright!")
                .para("Gather 'round crew. I have our heading, and intent."),
        )
        .line(Dialogue::new(CAPTAIN).para("We sail to find this great treasure."))
        .line(Dialogue::new(CREW3).para("And what is it???"))
        .line(Dialogue::new(CAPTAIN).para("King Triton's Trident"))
        .line(Dialogue::new(CREW1).para("A great treasure indeed! When do we leave?"))
        .line(
            Dialogue::new(CAPTAIN)
                .para("Right now, if the weather permits.")
                .para("What say ye sage?"),
        )
        .choice("Embark!", embark)
}

fn explore_shady_cove(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: shady_cove_treasure(),
        certainty: Certainty::Certain,
        distance: 10,
        environment: Environment::Island,
    });
    actions.add_dialogue(Dialogue::new(CAPTAIN).para("Adventure awaits!").clone());
    actions.change_environment(Environment::Island);
}

pub fn visit_shady_cove() -> DayEvent {
    DayEvent::new()
        .line(Dialogue::new(CAPTAIN).para("Shiver me timbers! Is that shady cove?"))
        .choice("Explore", explore_shady_cove)
}

fn take_shady_cove_treasure(journey: &mut StoryActions) {
    journey.delta_max_crew(-3);
    journey.delta_crew(-3);
    journey.delta_max_health(-3);
    journey.delta_health(-3);
    journey.add_dialogue(Dialogue::new(CAPTAIN).para("!"));
    journey.delta_items(Item::MonkeyPaw, 1);
}

pub fn shady_cove_treasure() -> DayEvent {
    DayEvent::new()
        .line(Dialogue::new(CAPTAIN).para("Arghh! Thats the treasure of shady cove!"))
        .choice("Take", take_shady_cove_treasure)
}
