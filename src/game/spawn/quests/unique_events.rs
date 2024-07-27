use super::{
    northern_seas::set_course_northern_sea,
    prelude::*,
    sea_events::{bounty_hunters, set_next_port},
};

fn embark(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: first_spotted_shady_cove,
        certainty: Certainty::Certain,
        delay: Delay::Distance(20),
        environment: Environment::Sea(Sea::Intro),
    });

    set_next_port(actions, 20);

    actions.delta_items(Item::Gold, 200);
    actions.delta_items(Item::Cannon, 3);
    actions.delta_items(Item::MonkeyPaw, 3);
    actions.delta_items(Item::SirensCoveMap, 3);
    actions.delta_items(Item::SirensScale, 3);
    actions.delta_items(Item::NorthernSeaMap, 3);
    actions.add_dialogue(captain!("And so we go to shady cove!"));
    actions.change_environment(Environment::Sea(Sea::Intro));
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
    actions.add_dialogue(captain!(
        "Beg your pardon for rushing in like that.",
        "Had to escape the turn in the weather."
    ));
    if actions.danger() > 3 {
        actions.add_dialogue(dock_worker!(
            "No worries sir. Its a terrible squall for sure."
        ));
        actions.add_event(FollowingEvent {
            event: shady_cove,
            environment: Environment::Port(Port::ShadyCove),
            delay: Delay::None,
            certainty: Certainty::Certain,
        });
    } else {
        actions.add_dialogue(dock_worker!(
            "What bad weather?",
            "If your are scared of that little cloud there maybe you shouldn't be sailing the high seas!"
        ));
        actions.add_event(FollowingEvent {
            event: rude_shady_cove,
            environment: Environment::Port(Port::ShadyCove),
            delay: Delay::None,
            certainty: Certainty::Certain,
        });
    }
    actions.change_environment(Environment::Port(Port::ShadyCove));
}

fn proper_docking(actions: &mut StoryActions) {
    if actions.danger() > 3 {
        actions.add_dialogue(crew1!(
            "Look out!",
            "The wind is blowing us into those rocks!"
        ));
        actions.add_dialogue(dock_worker!(
            "That was a rough entrance, welcome to shady cove."
        ));
        actions.delta_health(-actions.danger());
    } else {
        actions.add_dialogue(dock_worker!("Welcome to shady cove!"));
    }
    actions.add_dialogue(captain!(
        "I will be back in a while lads, I need to have a conversation with the dock worker."
    ));
    actions.change_environment(Environment::Port(Port::ShadyCove));
    actions.add_event(FollowingEvent {
        event: shady_cove,
        environment: Environment::Port(Port::ShadyCove),
        delay: Delay::None,
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
            "Don't want to be caught out here waiting on protocol."
        ))
        .hint("Proper DOcking means GOOD TRADE *SQUAWK*")
        .choice("Rush", rush_to_dock)
        .choice("Standby", proper_docking)
}

fn wait(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        event: shady_cove,
        certainty: Certainty::Certain,
        delay: Delay::None,
        environment: Environment::Port(Port::ShadyCove),
    });
}

pub fn shady_cove_base(_journey: &mut StoryActions) -> DayEvent {
    DayEvent::new().choice("Wait", wait)
}

fn buy_map(actions: &mut StoryActions) {
    actions.delta_items(Item::NorthernSeaMap, 1);
    actions.delta_items(Item::Gold, -100);
    actions.add_dialogue(map_merchant!("Much obliged."));
    actions.add_event(FollowingEvent {
        delay: Delay::None,
        environment: Environment::Port(Port::ShadyCove),
        certainty: Certainty::Certain,
        event: set_course_northern_sea,
    });
}

fn buy_map_cheap(actions: &mut StoryActions) {
    actions.delta_items(Item::NorthernSeaMap, 1);
    actions.delta_items(Item::Gold, -50);
    actions.add_event(FollowingEvent {
        delay: Delay::None,
        environment: Environment::Port(Port::ShadyCove),
        certainty: Certainty::Certain,
        event: set_course_northern_sea,
    });
    actions.add_dialogue(map_merchant!("Much obliged."))
}

fn steal_the_map(actions: &mut StoryActions) {
    actions.delta_items(Item::NorthernSeaMap, 1);
    actions.add_event(FollowingEvent {
        delay: Delay::None,
        environment: Environment::Port(Port::ShadyCove),
        certainty: Certainty::Certain,
        event: set_course_northern_sea,
    });
    let clarity = actions.get_clarity();

    actions.add_dialogue(crew2!("...So anyway, there I was twirling my beard when I saw the largest whale you could imagine-"));
    actions.add_dialogue(crew3!(
        "It's true, the thing was as big as my thumb held at arms length"
    ));
    actions.add_dialogue(crew2!("Stop interrupting! What does that even mean?"));
    if clarity > 5 {
        actions.add_event(FollowingEvent {
            environment: Environment::Sea(Sea::Northern),
            delay: Delay::Distance(18),
            event: bounty_hunters,
            certainty: Certainty::Possible(2),
        });
        actions.add_dialogue(map_merchant!("Hey! What do you think you are doing?"));
        actions.add_dialogue(crew1!("Blast! I've been made, run for it lads!"));
        actions.add_dialogue(map_merchant!("Get back here! Guards!"));
        actions.add_dialogue(captain!("Make for the ship!"));
        actions.delta_crew(-1);
    } else {
        actions.add_dialogue(crew1!(
            "Lads there is no need bother this poor map maker with your tales. Lets get going."
        ));
        actions.add_dialogue(map_merchant!(
            "Thank you. I was barely able to follow that incoherant gibberish."
        ));
        actions.add_dialogue(captain!(format!("*whispering* Good job there {CREW1}")))
    }
    actions.delta_items(Item::NorthernSeaMap, 1);
}

pub fn shady_cove(journey: &mut StoryActions) -> DayEvent {
    shady_cove_base(journey)
        .line(captain!(format!("Right, the dock worker pointed us at {MAP_MERCHANT}, says he can help us find what we need.")))
        .line(crew2!("Let's go have a little sit down with them then."))
        .line(narrator!("The crew heads for the map merchant."))
        .line(captain!("Hello there map maker.", "We are in need of a map of the northern seas"))
        .line(map_merchant!("You're in luck, I have just what you need!"))
        .choice("Steal", steal_the_map)
        .conditional_choice("Buy", buy_map_cheap, journey.get_item(Item::Gold) >= 50)
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
        .hint("Can't STEAL in BrOAD Daylight!")
        // TODO: MAKE CHOICES for good cove
        .choice("Steal", steal_the_map)
        .conditional_choice("Buy", buy_map, journey.get_item(Item::Gold) >= 100)
}
