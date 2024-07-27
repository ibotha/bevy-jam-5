use super::prelude::*;

fn embark(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: visit_shady_cove(),
        certainty: Certainty::Certain,
        distance: 20,
        environment: Environment::Sea,
    });
    actions.delta_items(Item::Gold, 200);
    actions.delta_items(Item::Cannon, 3);
    actions.add_dialogue(captain!("And so we go to shady cove!"));
    actions.change_environment(Environment::Sea);
}

pub fn embark_event() -> DayEvent {
    DayEvent::new()
        .line(crew1!("Ahoy sage! Have ye heard the news?"))
        .line(crew1!(
            "Word about port is there is a great treasure on the horizon."
        ))
        .line(crew2!(
            "Aye, I heard the same.",
            "Reckon we got a shot at it? Harhar..."
        ))
        .line(captain!(
            "Alright!",
            "Gather 'round crew. I have our heading, and intent."
        ))
        .line(captain!("We sail to find this great treasure."))
        .line(crew3!("And what is it???"))
        .line(captain!("King Triton's Trident"))
        .line(crew1!("A great treasure indeed! When do we leave?"))
        .line(captain!(
            "Right now, if the weather permits.",
            "What say ye sage?"
        ))
        .choice("Embark!", embark)
}

fn explore_shady_cove(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: shady_cove_treasure(),
        certainty: Certainty::Certain,
        distance: 10,
        environment: Environment::Island,
    });
    actions.add_dialogue(captain!("Adventure awaits!"));
    actions.change_environment(Environment::Island);
}

pub fn visit_shady_cove() -> DayEvent {
    DayEvent::new()
        .line(captain!("Shiver me timbers! Is that shady cove?"))
        .choice("Explore", explore_shady_cove)
}

fn take_shady_cove_treasure(journey: &mut StoryActions) {
    journey.delta_max_crew(-3);
    journey.delta_crew(-3);
    journey.delta_max_health(-3);
    journey.delta_health(-3);
    journey.add_dialogue(captain!("!"));
    journey.delta_items(Item::MonkeyPaw, 1);
}

pub fn shady_cove_treasure() -> DayEvent {
    DayEvent::new()
        .line(captain!("Arghh! Thats the treasure of shady cove!"))
        .choice("Take", take_shady_cove_treasure)
}
