use rand::Rng;

use super::prelude::*;
pub mod burial_site;
pub mod cave_entrance;
pub mod final_encounter;
use burial_site::burial_site;

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

pub fn debug_entery(actions: &mut StoryActions) {
    actions.add_event(FollowingEvent {
        environment: actions.get_environment(),
        event: sighted_edge_of_the_world,
        delay: Delay::None,
        certainty: Certainty::Certain,
    });
    actions.delta_items(Item::SirensScale, 3);
    actions.delta_items(Item::SirensCoveMap, 3);
    actions.delta_items(Item::NorthernSeaMap, 3);
    actions.delta_items(Item::Journal, 3);
    actions.delta_items(Item::Gold, 600);
    actions.delta_items(Item::SirenKiller, 3);
    actions.delta_items(Item::MonkeyPaw, 3);
    actions.delta_items(Item::SirenChild, 3);
    actions.delta_items(Item::SirensScale, 3);
    actions.delta_items(Item::GreekFire, 1);
    actions.delta_items(Item::Cannon, 8);
    actions.delta_crew(4);
}

pub fn debug_sirens_cove() -> DayEvent {
    DayEvent::new()
        .line(dialogue!("DEBUG"; "DEBUG"))
        .choice("DEBUG", debug_entery)
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

pub fn enter_sirens_cove_island(actions: &mut StoryActions) {
    actions.change_environment(Environment::Island(Island::SirensCove));
    actions.add_event(FollowingEvent {
        event: burial_site,
        delay: Delay::Days(3),
        certainty: Certainty::Certain,
        environment: actions.get_environment(),
    });
}

pub fn candle_wax(actions: &mut StoryActions) {
    actions.add_dialogue(crew2!(
        "I knew you had your head screwed on straight, I will be back with the candles."
    ));
    actions.add_dialogue(narrator!(
        "They scurry downstairs, a short while later the entire crew has wax in their ears."
    ));
    actions.add_dialogue(crew2!("WE SHOULD BE SAFE NOW!"));
    actions.add_dialogue(captain!("LET'S TAKE HER IN LADS!"));
    actions.add_dialogue(narrator!("A little while later..."));
    match actions.weather().heat {
        H::Blistering | H::Warm => {
            actions.add_dialogue(crew1!("mm mup ee mumupo..."));
            actions.add_dialogue(crew3!("WHAT?? I CAN'T HEAR YOU!"));
            actions.add_dialogue(crew1!("I said, my wax is melting."));
            actions.add_dialogue(crew3!(
                "Oh... mine too. Say, I don't really feel like holding this rope anymore..."
            ));
            actions.add_dialogue(sirens!("Aah aah ahh haa..."));
            actions.add_dialogue(narrator!(
                "You don't remember what happened next, all you can do is assess the damages."
            ));
            actions.delta_items(Item::Cannon, -2);
            actions.delta_crew(-2);
            actions.delta_health(-10);
        }
        _ => {
            actions.add_dialogue(crew3!("THIS IS WORKING A TREAT!"));
            actions.add_dialogue(narrator!(
                "You make your way to a small beach, shielded from the song by a hill."
            ))
        }
    }
    enter_sirens_cove_island(actions);
}
pub fn rocky_shore(actions: &mut StoryActions) {
    actions.add_dialogue(crew2!("If Poseidon is on our side, we can make it!"));
    actions.add_dialogue(narrator!("You start toward the rocky shore."));
    if actions.danger() >= 3 {
        actions.add_dialogue(narrator!(
            "The seas are rough, but you do make it to shore."
        ));
        actions.delta_items(Item::SirensScale, -1);
        actions.delta_items(Item::Cannon, -2);
        actions.delta_items(Item::Gold, -200);
        actions.delta_crew(-2);
    } else {
        actions.add_dialogue(narrator!(
            "Smooth sailing, you make your way to the rocky shore."
        ));
    }
    enter_sirens_cove_island(actions);
}
pub fn trust_the_wind(actions: &mut StoryActions) {
    actions.add_dialogue(crew3!(
        "The sage says the wind is in our favor! Everyone get below decks, I will get the rope!"
    ));
    actions.add_dialogue(narrator!(
        format!("They scurry downstairs, a short while later everyone is immobilised apart from {CREW3}, who is diligently covering his ears.")
    ));
    actions.add_dialogue(sirens!("Aah aah ahh haa..."));
    actions.add_dialogue(crew2!(
        "I must get above deck, that voice... it is one of an angel."
    ));
    actions.add_dialogue(captain!(format!(
        "Aye, perhaps this rope was a bad idea {CREW3}, what say you let us out?"
    )));
    actions.add_dialogue(narrator!(format!("{CREW3}, however, can't hear a word.")));
    actions.add_dialogue(sirens!("...haa aah ahh haa..."));
    actions.add_dialogue(crew1!("LET US OUT NOW!"));
    actions.add_dialogue(narrator!("The next while chaos ensues as the crew desperately try to escape their surprisingly well-tied restraints."));
    actions.add_dialogue(crew1!(
        "It's coming loose haha! Soon I will be with you my love!"
    ));
    match actions.weather().wind {
        W::High | W::GaleForce => {
            actions.add_dialogue(narrator!("The siren's song begins to get softer."));
            actions.add_dialogue(crew1!("I will get out of here- wait... where am I?"));
            actions.add_dialogue(captain!("We have our whits about us now. Let's go ashore."));
            actions.add_dialogue(narrator!(
                "You find yourselves on a small beach, shielded from the song by a hill."
            ))
        }
        _ => {
            actions.add_dialogue(narrator!(
                format!("{CREW1} escapes and chaos ensues."),
                "You don't remember what happened next, all you can do is assess the damages."
            ));
            actions.delta_items(Item::Cannon, -2);
            actions.delta_crew(-2);
            actions.delta_health(-10);
        }
    }
    enter_sirens_cove_island(actions);
}

pub fn sighted_sirens_cove(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!(
            "Do you hear that? That is the most beautiful sound I have ever heard..."
        ))
        .line(crew2!("Aye reminds me of a cool sea breeze."))
        .line(captain!("It's siren song, cover your ears lads, while its still faint."))
        .line(crew1!("I nearly lost myself there already, how are we supposed to get to the island like this?"))
        .line(crew2!("There is a whole stock of candles downstairs, I say we fill our ears with wax!"))
        .line(crew1!("There is another shore there, rocky as hell but should be far enough away from the song to allow us to get inland."))
        .line(crew3!("Looks treacherous, we would need clear skies to navigate that."))
        .line(crew3!("If the wind is high enough we could set the ship on course and tie ourselves down, we might get in before we escape our restraints."))
        .line(captain!("What say ye sage?"))
        .choice("Candle Wax", candle_wax)
        .choice("Far Rocky Shore", rocky_shore)
        .choice("Trust the Wind", trust_the_wind)
}
