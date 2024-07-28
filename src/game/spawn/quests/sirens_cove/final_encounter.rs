use rand::Rng;

use crate::game::spawn::quests::prelude::*;

fn victory(actions: &mut StoryActions) {
    actions.add_dialogue(captain!("We did it, King Triton's trident is ours!!!"));
    actions.add_dialogue(captain!("The seas and the skies are now ours to control!"));
    actions.add_dialogue(captain!("And it's all thanks to you, my trusty sage."));
    actions.game_over();
}

fn attack_inner(actions: &mut StoryActions, siren_bonus: i32) {
    let siren_strength = 8
        + siren_bonus
        + match actions.weather().moisture {
            M::Dry => -1,
            M::Comfortable => 0,
            M::Humid => 1,
        };
    let bonuses = actions.get_item(Item::GreekFire) + actions.get_item(Item::SirenKiller);
    actions.add_dialogue(captain!("Get em lads!!!"));
    actions.add_dialogue(sirens!(
        "*bursting out into a vicious melody* AAAH HAA AHHHA!"
    ));
    actions.add_dialogue(crew!("AAARGH!"));
    actions.battle(siren_strength, bonuses, "King Triton");
    if actions.get_crew() > 0 {
        actions.add_dialogue(captain!("*Panting* That's the last of them."));
        victory(actions);
    }
}

fn attack(actions: &mut StoryActions) {
    attack_inner(actions, 0);
}

fn bargain(actions: &mut StoryActions) {
    actions.add_dialogue(captain!(
        "KING TRITON! If you are here I would like to have a word!"
    ));
    actions.add_dialogue(captain!("And do try anything funny... I have someone here that I think we would all like to see get out of here unscathed"));
    if actions.get_rng().gen_range(0..10) < 3 {
        actions.add_dialogue(king_triton!(
            "How DARE you take my daughter! You fools will die here!",
            "Sirens, ATTACK!"
        ));
        attack_inner(actions, 2);
    } else {
        actions.add_dialogue(king_triton!(
            "My dear daughter! *He wails* You need to be more careful!"
        ));
        actions.add_dialogue(king_triton!("What do you want you monster!"));
        actions.add_dialogue(captain!(
            "Your trident. I want the power to command the seas."
        ));
        actions.add_dialogue(king_triton!("...Fine! Take it."));
        actions.add_dialogue(narrator!("He throws his trident at the captain's feet with godly force. It slams against the captain's boots"));
        actions.add_dialogue(captain!(
            "We will be taking our leave now, pleasure doing business with you."
        ));
        victory(actions);
    }
}

fn home(actions: &mut StoryActions) {
    actions.add_dialogue(narrator!(
        "You head on your way, back out of the cove and onto the high seas."
    ));
    actions.add_dialogue(narrator!(
        "Perhaps the real treasure was the friends we made along the way."
    ));
    actions.game_over();
}

fn steal(actions: &mut StoryActions) {
    actions.add_dialogue(captain!("Easy does it lads, lets make this happen."));
    if actions
        .get_rng()
        .gen_range(0..10 - actions.get_item(Item::SirensScale))
        > 3
    {
        actions.add_dialogue(king_triton!(
            "Stupid pirates, sneaking into my lair and trying to steal my trident! You fools will die here!",
            "Sirens, ATTACK!"
        ));
        attack_inner(actions, 2);
    } else {
        actions.add_dialogue(narrator!(
            "It works! king triton is distracted by the singing of some of the sirens."
        ));
        victory(actions);
    }
}

pub fn final_encounter(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("We're here. Our prize awaits."))
        .line(captain!("Don't be hasty. Let's weigh our options."))
        .line(crew1!("Are we strong enough to fight them?"))
        .c_line(
            crew1!("We have Greek Fire. Siren's are vulnerable to fire."),
            actions.get_item(Item::GreekFire) > 0,
        )
        .c_line(crew1!(
            "We know how to kill them. The Pirate Journal told us that siren's are weaker if it is dry out!"
        ), actions.get_item(Item::Journal) > 0)
        .c_line(crew2!(
            "We have the siren killer!"
        ),  actions.get_item(Item::SirenKiller) > 0)
        .line(crew3!("Any options where we don't have to fight deadly beasts?"))
        .c_line(crew2!(
            "We have the child, we could bargain for the trident."
        ), actions.get_item(Item::SirenChild) > 0)
        .line(crew1!(
            "We could try to sneak and steal the trident."))
        .c_line(crew2!(
            "I can go with Patchy and the scales will alert us to the presance of sirens.",
            "You take the crew and cause a distraction!"
        ), actions.get_item(Item::Journal) > 0)
        .line(crew3!(
            "They look pretty scary... we could just go home. Sage has proven they're as good as the trident."))
        .line(crew3!(
            "We don't need it. We can keep our lives."
        ))
        .line(crew2!(
            "Coward, we can't stop when we are this close!"
        ))
        .line(captain!("Heavy decisions. What do you think is best Sage?"))
        .choice("Bargain", bargain)
        .choice("Attack", attack)
        .choice("Go Home", home)
        .choice("Steal", steal)
}
