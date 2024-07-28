use super::sea_stories_base;
use crate::game::spawn::quests::prelude::*;

fn traverse_celestial_path(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 200);
    actions.delta_health(5);
    actions.delta_crew(2);
    actions.delta_items(Item::Cannon, 1);
    actions.add_dialogue(captain!("Inconceivable! We've traversed the Celestial Path! Our ship has been transformed into a divine vessel, our crew has ascended to demigod status, and we've acquired treasures that could buy entire kingdoms!"));
}

fn commune_with_sea_deities(actions: &mut StoryActions) {
    if actions.get_item(Item::MonkeyPaw) > 0 {
        actions.delta_crew(2);
        actions.delta_health(15);
        actions.delta_items(Item::Gold, 150);
        actions.delta_items(Item::Cannon, 1);
        actions.add_dialogue(captain!("The Monkey's Paw has transformed into a divine artifact! We've formed an alliance with the sea deities themselves. Our ship can now command the oceans at will, and we've been granted the power to reshape coastlines and summon storms!"));
    } else {
        actions.delta_crew(2);
        actions.delta_health(15);
        actions.delta_items(Item::Gold, 100);
        actions.delta_items(Item::Cannon, 1);
        actions.add_dialogue(captain!("We've communed with the sea deities and received their blessings. Our ship can now breathe underwater and control sea creatures. We've been gifted with divine artifacts and the power to calm or incite storms at will."));
    }
}

fn collect_celestial_waters(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 50);
    actions.delta_health(5);
    actions.delta_crew(1);
    actions.delta_items(Item::Cannon, 1);
    actions.add_dialogue(captain!("We've collected vials of the celestial waters revealed by the sea parting. Even this small amount has imbued our ship with incredible properties. Our crew now possesses enhanced strength and longevity, and the waters themselves are worth more than gold!"));
}

pub fn the_celestial_sea_parting_event(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("Cap'n! The sea... it's splitting open! There's a glowing path stretching to the horizon!"))
        .line(captain!("Great Neptune's ghost! I've never seen anything like this in all my years at sea!"))
        .line(crew2!("The waters are rising like walls on either side, sir! And the exposed seabed... it's shimmering with unearthly light!"))
        .line(crew3!("It's the Celestial Sea Parting, Cap'n! A phenomenon said to occur once in a thousand years when the gods themselves walk the ocean floor!"))
        .line(captain!("The Celestial Sea Parting... I thought it was just a myth. This is a once-in-a-lifetime opportunity that could change everything. What course shall we chart in this divine moment?"))
        .choice("Traverse Path", traverse_celestial_path)
        .conditional_choice("Commune", commune_with_sea_deities, actions.get_item(Item::MonkeyPaw) > 0)
        .choice("Collect Waters", collect_celestial_waters)
        .hint("Squawk! When the sea parts, legends are born!")
}