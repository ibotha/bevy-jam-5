use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn embrace_the_convergence(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 1000);
    actions.delta_health(30);
    actions.delta_crew(25);
    actions.delta_items(Item::Cannon, 50);
    actions.add_dialogue(captain!("By all that's holy and unholy! We've become legends beyond imagination! Our ship has transformed into a living, interdimensional entity. We command the powers of gods, our crew includes beings from across the multiverse, and our wealth is beyond mortal comprehension!"));
}

fn negotiate_with_cosmic_entities(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(25);
        actions.delta_health(50);
        actions.delta_items(Item::Gold, 25);
        actions.delta_items(Item::Cannon, 4);
        actions.add_dialogue(captain!("The Monkey's Paw has ascended to a cosmic artifact! We've forged alliances with beings beyond comprehension. Our ship can now rewrite reality at will, and we've been granted stewardship over multiple dimensions!"));
    } else {
        actions.delta_crew(10);
        actions.delta_health(20);
        actions.delta_items(Item::Gold, 500);
        actions.delta_items(Item::Cannon, 3);
        actions.add_dialogue(captain!("We've managed to negotiate with cosmic forces beyond our understanding. While we couldn't fully harness the Convergence, we've still gained immense power and knowledge. Our ship can now traverse dimensions, and we've been gifted with cosmic artifacts of immense power."));
    }
}

fn observe_and_document(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 300);
    actions.delta_health(10);
    actions.delta_crew(5);
    actions.delta_items(Item::Cannon, 2);
    actions.add_dialogue(captain!("We chose to observe the Convergence from a safe distance. While we didn't directly interact with it, the sheer proximity has imbued our ship and crew with incredible power. We've documented knowledge that will revolutionize the world, and collected artifacts that defy explanation."));
}

pub fn the_convergence_of_realms_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The world... it's... it's unraveling! I can see other realities bleeding into ours!"))
        .line(captain!("Steady, crew! This is beyond anything we've ever encountered!"))
        .line(crew2!("It's as if all of existence is converging on this point, sir! I can see impossible landscapes, creatures of myth, and technologies beyond imagination!"))
        .line(crew3!("It's the Convergence of Realms, Cap'n! A cosmic event where all realities, all possibilities, all of existence meets at a single point! It's said to happen once in the lifetime of the universe itself!"))
        .line(captain!("The Convergence of Realms... This is our one chance to transcend everything we've ever known. The fate of not just our world, but all worlds, hangs in the balance. What shall we do in this moment of cosmic significance?"))
        .choice("Embrace Convergence", embrace_the_convergence)
        .conditional_choice("Negotiate", negotiate_with_cosmic_entities, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Observe", observe_and_document)
        .hint("Squawk! In this moment, we stand at the crossroads of all existence!")
}