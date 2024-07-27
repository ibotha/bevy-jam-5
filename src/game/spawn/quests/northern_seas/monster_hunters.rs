use crate::game::spawn::quests::sea_stories::sail;

use super::super::prelude::*;

fn parlay(actions: &mut StoryActions) {
    actions.add_dialogue(captain!("Ahoy there! What colours do you fly?"));
    actions.add_dialogue(monster_hunter!(
        "The flag of the monster hunter's guild.",
        "Who goes there?"
    ));
    actions.add_dialogue(captain!(
        "Captain Snap-Patch at yer service.",
        "May we come aboard?"
    ));
    actions.add_dialogue(monster_hunter!("The more the merrier, lets have a feast!"));
    actions.add_dialogue(narrator!("Your crew tie off to the monster hunter ship, they bring out the ale and a lute while the captains speak."));
    actions.add_dialogue(monster_hunter!("So what brings you out here?"));
    actions.add_dialogue(monster_hunter!("And who is this?"));
    actions.add_dialogue(captain!(
        "That is my trusty sage, you need all the help you can get in these waters."
    ));
    actions.add_dialogue(monster_hunter!("Indeed."));
    actions.add_dialogue(captain!("We are looking for king Triton's trident."));
    actions.add_dialogue(monster_hunter!(
        "Ha! I had heard the rumours to, but only a fool would try to steal from a siren's nest."
    ));
    actions.add_dialogue(captain!("You mean \"Siren's Cove\" isn't just a name?"));
    actions.add_dialogue(monster_hunter!("Most certainly not."));
    actions.add_dialogue(captain!(
        "Well that won't stop us right lads? We will just have to be ready!"
    ));
    actions.add_dialogue(monster_hunter!(
        "Haha! *to everyone* Cheers! To your bravery, friends."
    ));
    actions.add_dialogue(crew!("Cheers!"));
    actions.add_event(FollowingEvent {
        delay: Delay::None,
        event: a_monstrous_proposition,
        certainty: Certainty::Certain,
        environment: Environment::Sea(Sea::Northern),
    });
}

fn plunder(actions: &mut StoryActions) {
    let favour = actions.battle(7, 0, "Monster Hunters");
    actions.delta_items(Item::Cannon, favour / 3);
    actions.delta_items(Item::Gold, favour * 10);
    actions.delta_items(Item::SirenKiller, 1);
}

pub fn monster_hunters(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("A ship Captain, a big one at that!"))
        .line(captain!("Do you reckognise the colours?"))
        .line(crew1!("No sir, looks like a kraken on their flag."))
        .choice("Parlay", parlay)
        .choice("Plunder", plunder)
        .choice("Ignore", sail)
}

fn fight_the_monster(actions: &mut StoryActions) {
    let favour = actions.battle(2 + actions.danger(), 0, "Umibozu");
    actions.delta_items(Item::Cannon, (favour / 3).min(0));
    actions.delta_items(Item::Gold, (favour * 10).max(0));
    actions.delta_items(Item::SirenKiller, 1);
}

pub fn a_monstrous_proposition(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!("Sage, the monster hunter told me something last night.", "He said they could use a hand with the prey they are after now."))
        .line(captain!("They know their heading, he wants us to follow them and help to surround the creature."))
        .line(captain!("What say ye? Will we have clear weather for a fight?"))
        .choice("Fight", fight_the_monster)
        .choice("Sail On", sail)
}
