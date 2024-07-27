use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn ascend_island(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Dry) => {
            actions.delta_items(Item::Gold, 15000);
            actions.delta_items(Item::Cannon, 10);
            actions.delta_health(200);
            actions.delta_crew(20);
            actions.add_dialogue(captain!("Beyond imagination! We've reached the summit and been granted power by the celestial beings! Our ship is now imbued with cosmic energy, our cannons fire starlight, and our crew has been elevated to demigod status. We're no longer mere pirates, we're legends incarnate!"));
        }
        (H::Warm, W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 8000);
            actions.delta_items(Item::Cannon, 5);
            actions.delta_crew(-3);
            actions.delta_health(100);
            actions.add_dialogue(captain!("We've reached the celestial realm, but the ascension was too much for some. We lost three crew to the transformation, but those who remained are now infused with cosmic power. Our ship sails through dimensions, and our holds overflow with stardust gold."));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_crew(-10);
            actions.delta_health(-100);
            actions.delta_items(Item::Gold, 3000);
            actions.add_dialogue(captain!("Calamity! The harsh conditions angered the celestial beings. Half our crew vanished into cosmic dust, and our ship is barely clinging to this plane of existence. We managed to grab some celestial gold as we fled, but at what cost to our souls?"));
        }
        _ => {
            actions.delta_items(Item::Gold, 5000);
            actions.delta_items(Item::Cannon, 3);
            actions.delta_crew(5);
            actions.delta_health(50);
            actions.add_dialogue(captain!("We've touched the face of eternity and returned changed. Our ship now hums with ethereal energy, our new cannons shoot bolts of lightning, and some of the crew have developed... unusual abilities. The cosmos has opened its coffers to us, but I fear we've only glimpsed the true power that exists beyond our world."));
        }
    }
}

fn commune_with_celestials(actions: &mut StoryActions) {
    if actions.get_item(Item::Gold) >= 5000 {
        actions.delta_items(Item::Gold, -5000);
        actions.delta_items(Item::Cannon, 7);
        actions.delta_health(150);
        actions.delta_crew(10);
        actions.add_dialogue(captain!("Our offering pleased the celestial beings! They've granted us knowledge beyond mortal ken. Our ship can now sail the stars themselves, our new celestial cannons defy the laws of physics, and our crew has been blessed with immortality. The gold was but a small price for godhood!"));
    } else {
        actions.delta_health(-80);
        actions.delta_crew(-5);
        actions.delta_items(Item::Gold, 2000);
        actions.add_dialogue(captain!("The celestials found our offering insufficient! They've cursed us with cosmic awareness beyond our comprehension. We've lost crew to madness, and our ship groans with eldritch energies. They left us with some pity gold, but was it worth the sanity we've lost?"));
    }
}

fn observe_from_afar(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 2000);
    actions.delta_items(Item::Cannon, 1);
    actions.delta_health(30);
    actions.delta_crew(2);
    actions.add_dialogue(captain!("We kept our distance, observing the celestial island through spyglasses. Even from afar, its power has changed us. Gold rained from the sky into our holds, a cosmic cannon materialized on our deck, and two ethereal beings have joined our crew out of curiosity. Our journey will never be the same, even if we didn't set foot on that impossible island."));
}

pub fn the_celestial_island_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("CAP'N! There's an island ahead... but it's floating in the sky!"))
        .line(captain!("Impossible... Could it be the legendary Celestial Island?"))
        .line(crew2!("It's real, Cap'n! I see beings of pure light moving about up there!"))
        .line(crew3!("They say those who reach the top are granted power over reality itself... but the journey could unmake us."))
        .line(captain!("This is beyond pirates, beyond mortals. Do we dare to reach for divinity, or is that power not meant for the likes of us?"))
        .choice("Ascend Island", ascend_island)
        .conditional_choice("Commune", commune_with_celestials, actions.get_item(Item::Gold) >= 5000)
        .choice("Observe Afar", observe_from_afar)
        .hint("Squawk! Even the heavens aren't out of reach for a determined pirate!")
}