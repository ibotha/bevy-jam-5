use rand::Rng;

use super::prelude::*;

pub fn to_sirens_cove(actions: &mut StoryActions) {
    let days_until_encounter = actions
        .get_rng()
        .gen_range(3..(10 - actions.get_item(Item::SirensScale)).max(3));
    actions.add_dialogue(captain!(
        "Aye, I too feel we are ready.",
        "For glory! For the sea! And for all the gold you could fill your boots with! Hahahar"
    ));
    actions.change_environment(Environment::Sea(Sea::SirensCove));
    actions.add_event(FollowingEvent {
        event: sighted_sirens_cove,
        delay: Delay::Days(days_until_encounter),
        certainty: Certainty::Certain,
        environment: Environment::Sea(Sea::SirensCove),
    })
}

pub fn no(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea(Sea::Northern));
}

pub fn sighted_edge_of_the_world(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!(
            "We stand at the precipice of our greatest prize!",
            "Is now the time to enter siren's cove? Once we go in there is no turning back."
        ))
        .choice("Yes", to_sirens_cove)
        .choice("No", no)
}

pub fn sighted_sirens_cove(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!(
            "Do you hear that? That is the most beautiful sound I have ever heard..."
        ))
        .line(crew2!("Aye reminds me of a cool sea breeze."))
        .choice("Yes", to_sirens_cove)
        .choice("No", no)
}
