use super::{super::prelude::*, set_course_northern_sea};

fn purchase_paws(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -50);
    actions.delta_items(Item::MonkeyPaw, 3);
}

fn purchase_sword(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -150);
    actions.delta_items(Item::SirenKiller, 3);
}

fn purchase_greek_fire(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -300);
    actions.delta_items(Item::GreekFire, 1);
}

pub fn sighted_trinket_seller(actions: &mut StoryActions) -> DayEvent {
    set_course_northern_sea(actions)
        .line(narrator!("You arrive at the trinket seller."))
        .line(captain!("Have anything that can help with sirens?"))
        .line(trinket_seller!("As it happens I do, but it won't be cheap. I can make you immune to the cold for 300 gold" ))
        .line(trinket_seller!(if actions.get_item(Item::SirenKiller) == 0 {"As it happens, I also have this blade I retrieved from a nearby shipwreck. Only 150 gold."} else {"other than tha..."}))
        .line(trinket_seller!("I have magical aids if you are in need. 50 gold for 3"))
        .conditional_choice("Monkey Paws", purchase_paws, actions.get_item(Item::Gold) >= 50)
        .conditional_choice("Greek Fire", purchase_greek_fire, actions.get_item(Item::Gold) >= 300)
        .conditional_choice("Sword", purchase_sword, actions.get_item(Item::Gold) >= 150 && actions.get_item(Item::SirenKiller) == 0)
}
