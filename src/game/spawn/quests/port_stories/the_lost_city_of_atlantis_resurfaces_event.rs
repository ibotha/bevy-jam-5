use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn explore_atlantis(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 400 && actions.get_crew() >= 8 {
        actions.delta_items(Item::Gold, -400);
        actions.delta_crew(-2);
        actions.delta_health(-20);
        actions.delta_items(Item::Gold, 1500);
        actions.delta_items(Item::MonkeyPaw, 1);
        actions.add_dialogue(captain!("By Neptune's beard, we've done it! We've explored the depths of Atlantis and returned with untold riches and mystical artifacts. We lost two brave souls to the city's defenses, but our names will be etched in history!"));
        // We can potentially add a future event
    } else {
        actions.delta_items(Item::Gold, -150);
        actions.delta_health(-15);
        actions.delta_crew(-1);
        actions.add_dialogue(captain!("We weren't prepared for the perils of Atlantis. We barely escaped with our lives, losing gold, a crew member, and damaging our ship in the process."));
    }
}

fn document_discovery(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -100);
    actions.delta_food(-15);
    actions.delta_items(Item::Gold, 300);
    actions.add_dialogue(captain!("We meticulously documented the city's reappearance from a safe distance. Scholars paid handsomely for our detailed records and maps. This knowledge could revolutionize our understanding of ancient civilizations."));
    // We can potentially add a future event
}

fn warn_authorities(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 200);
    actions.delta_crew(3);
    actions.add_dialogue(captain!("We alerted the authorities about the potential dangers of Atlantis. They rewarded us for our caution and civic duty. Our reputation for wisdom has attracted new crew members seeking to learn from us."));
}

pub fn the_lost_city_of_atlantis_resurfaces_event(actions: &mut StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! You're not going to believe this, but Atlantis has resurfaced off the coast!"))
        .line(captain!("Atlantis? The mythical sunken city? Are you sure your eyes aren't playing tricks on you?"))
        .line(crew2!("It's true, Cap'n! The whole port's in an uproar. They say it's filled with ancient treasures and technology beyond our wildest dreams."))
        .line(crew3!("Aye, but it's also said to be protected by ancient magics and treacherous defenses. Many are too afraid to go near it."))
        .line(captain!("The lost city of Atlantis... This could be the discovery of a lifetime, or a quick path to a watery grave. What are our options?"))
        .conditional_choice("Explore", explore_atlantis, actions.get_item(Item::Gold) >= 150 && actions.get_crew() >= 8)
        .choice("Document discovery", document_discovery)
        .choice("Warn", warn_authorities)
        .hint("Squawk! The greatest treasures often lie in the most perilous depths!")
}
