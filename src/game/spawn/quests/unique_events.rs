use super::{
    prelude::*,
    sea_events::{evaluate_sea_weather, set_next_port},
};

fn embark(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: first_spotted_shady_cove,
        certainty: Certainty::Certain,
        distance: 20,
        environment: Environment::Sea,
    });

    set_next_port(actions, 20);

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

fn rush_to_dock(actions: &mut StoryActions) {
    actions.change_environment(Environment::Port);
    let (_, danger) = evaluate_sea_weather(actions);
    if danger > 5 {
        actions.add_dialogue(captain!(
            "Beg your pardon for rushing in like that.",
            "Had to escape the turn in the weather"
        ));
        actions.add_dialogue(dock_worker!(
            "No worries sir. Its a terrible squall for sure."
        ));
        actions.add_event(FollowingEvent {
            event: shady_cove,
            environment: Environment::Port,
            distance: 0,
            certainty: Certainty::Certain,
        });
    } else {
        actions.add_dialogue(captain!(
            "Beg your pardon for rushing in like that.",
            "Had to escape the turn in the weather"
        ));
        actions.add_dialogue(dock_worker!(
            "No worries sir. Its a terrible squall for sure."
        ));
        actions.add_event(FollowingEvent {
            event: rude_shady_cove,
            environment: Environment::Port,
            distance: 0,
            certainty: Certainty::Certain,
        });
    }
}

fn proper_docking(actions: &mut StoryActions) {
    actions.change_environment(Environment::Port);
    let (_, danger) = evaluate_sea_weather(actions);
    if danger > 5 {
        actions.add_dialogue(crew1!(
            "Look out!",
            "The wind is blowing us into those rocks!"
        ));
        actions.add_dialogue(dock_worker!(
            "No worries sir. Its a terrible squall for sure."
        ));
    } else {
        actions.add_dialogue(captain!(
            "Beg your pardon for rushing in like that.",
            "Had to escape the turn in the weather"
        ));
        actions.add_dialogue(dock_worker!(
            "No worries sir. Its a terrible squall for sure."
        ));
    }

    actions.add_event(FollowingEvent {
        event: shady_cove,
        environment: Environment::Port,
        distance: 0,
        certainty: Certainty::Certain,
    });
}

pub fn first_spotted_shady_cove(_journey: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew2!("Shady Cove on the horizon"))
        .line(captain!("Bring us in lads!"))
        .line(crew!("Aye Aye! Captain."))
        .line(crew2!("I'll signal the docks."))
        .line(captain!(
            "How's the weather Sage?",
            "Don't want to be caught out here waiting on protocol"
        ))
        .hint("Proper DOcking means GOOD TRADE *SQUAWK*")
        .choice("Rush", rush_to_dock)
        .choice("Standby", proper_docking)
}

fn wait(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: shady_cove,
        certainty: Certainty::Certain,
        distance: 0,
        environment: Environment::Port,
    });
}

pub fn shady_cove_base(_journey: &mut StoryActions) -> DayEvent {
    DayEvent::new().choice("Wait", wait)
}

pub fn shady_cove(journey: &mut StoryActions) -> DayEvent {
    shady_cove_base(journey)
        .line(captain!(format!("Right, the dock worker pointed us at {MAP_MERCHANT}, says he can help us find what we need.")))
        .line(crew2!("Let's go have a little sit down with them then."))
        .line(narrator!("The crew heads for the map merchant."))
        .line(captain!("Hello there map maker.", "We are in need of a map of the northern seas"))
        .line(map_merchant!("You're in luck, I have just what you need!"))
    // TODO: MAKE CHOICES for good cove
}

pub fn rude_shady_cove(journey: &mut StoryActions) -> DayEvent {
    shady_cove_base(journey)
        .line(crew2!("Folks don't seem very forthcoming."))
        .line(captain!("Our little stunt didn't make us any friends..."))
        .line(crew1!("If we wait a while they might calm down."))
        .line(crew2!(
            "I was able to find a map at least, provided we get the money for it."
        ))
        .line(crew3!("We could always try steal it..."))
        .line(crew2!("I'll signal the docks."))
        .line(captain!(
            "How's the weather Sage?",
            "Don't want to be caught out here waiting on protocol"
        ))
        .hint("Can't STEAL in BrOAD Daylight!")
    // TODO: MAKE CHOICES for good cove
    // .choice("Steal", steal_the_map)
    // .choice("Buy", buy_map)
}
