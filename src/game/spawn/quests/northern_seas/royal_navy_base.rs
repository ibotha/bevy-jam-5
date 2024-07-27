use super::{super::prelude::*, set_course_northern_sea};

fn buy(actions: &mut StoryActions) {
    actions.delta_items(Item::SirensCoveMap, 1);
}

fn break_in(actions: &mut StoryActions) {
    actions.delta_items(Item::SirensCoveMap, 1);
    actions.add_dialogue(captain!("Around the back, there is a sewer entrance."));
    actions.add_dialogue(narrator!(
        "Like a well-oiled machine the crew systematically breaks through barrier after barrier."
    ));
    actions.add_dialogue(captain!("One last gate and we are in the main keep."));
    if actions.get_clarity() < 5 {
        actions.add_dialogue(crew1!("We are in! And they are none the wiser."));
        actions.add_dialogue(prisoner!(
            "How kind of you, that gate was starting to annoy me."
        ));
        actions.add_dialogue(prisoner!(
            "How kind of you, that gate was starting to annoy me."
        ));
    } else {
        actions.add_dialogue(crew1!("Oh boy! We have been spotted. "));
    }
}

fn blow_it_up(actions: &mut StoryActions) {
    actions.delta_items(Item::SirensCoveMap, 1);
}

pub fn sighted_navy_base(actions: &mut StoryActions) -> DayEvent {
    actions.change_environment(Environment::Port(Port::RoyalNavyBase));
    set_course_northern_sea(actions)
        .line(crew1!("Royal Navy base sighted!"))
        .line(captain!("Aye, lets bring her in."))
        .line(narrator!("A little while later after docking..."))
        .line(crew2!("What's the plan captain, how are we going to get the map? An official chart from the royal navy will run us down 300 gold!"))
        .line(captain!(if actions.get_item(Item::Gold) >= 300 {"Well, I suppose we could just buy it if we like..."} else {"We can't afford that..."}))
        .line(crew3!("We could try break in!"))
        .line(crew1!("Maybe if we had some cover fog..."))
        .line(crew2!("Or if it is dry out we could use some gunpowder barrels.", "Kaboom!"))
        .conditional_choice("Buy", buy,actions.get_item(Item::Gold) >= 300)
        .choice("Break In", break_in)
        .choice("Gunpowder!", blow_it_up)
}
