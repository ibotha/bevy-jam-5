use crate::game::spawn::quests::prelude::*;
use super::port_stories_base;

fn accept_drinking_contest(actions: &mut StoryActions) {
    let DW {
        heat,
        wind: _,
        moisture: _,
    } = actions.weather();

    actions.delta_items(Item::Gold, -50); // Entry fee for the contest

    match heat {
        H::Blistering | H::Warm => {
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The heat got to me. We lost the contest and our entry fee, and I've got a splitting headache."));
        }
        H::Comfortable => {
            actions.delta_items(Item::Gold, 150); // 100 profit
            actions.delta_crew(1);
            actions.add_dialogue(captain!("Ha! We showed them how it's done. Won some gold and impressed a new crew member with our victory!"));
        }
        H::Chilly | H::Freezing => {
            actions.delta_items(Item::Gold, 100); // 50 profit
            actions.delta_food(10);
            actions.add_dialogue(captain!("The cold helped us keep our heads. We won some gold and got a round of hot meals from the loser."));
        }
    }
}

fn accept_navigation_challenge(actions: &mut StoryActions) {
    let DW {
        heat: _,
        wind,
        moisture: _,
    } = actions.weather();

    actions.delta_items(Item::Gold, -75); // Entry fee for the challenge

    match wind {
        W::None | W::Low => {
            actions.delta_items(Item::Gold, 225); // 150 profit
            actions.add_dialogue(captain!("With calm winds, our navigation skills shone. We won handily and earned a tidy sum."));
        }
        W::Medium => {
            actions.delta_items(Item::Gold, 150); // 75 profit
            actions.delta_health(5);
            actions.add_dialogue(captain!("A close contest! We just edged out a victory, winning some gold and respect for our ship handling."));
        }
        W::High | W::GaleForce => {
            actions.delta_health(-5);
            actions.add_dialogue(captain!("The wild winds made it nearly impossible. We lost the challenge and our entry fee, and took some damage, but it was a learning experience."));
        }
    }
}

fn decline_challenge(actions: &mut StoryActions) {
    actions.delta_crew(-1);
    actions.delta_items(Item::Gold, 25);
    actions.add_dialogue(captain!("We avoided the risk, but lost face. One crew member left, ashamed, but another captain paid us to carry some cargo as a consolation."));
}

pub fn the_rival_captains_challenge_event(actions: &StoryActions) -> DayEvent {
    port_stories_base(actions)
        .line(crew1!("Cap'n! There's another captain at the tavern, boasting about being the best on the seas."))
        .line(captain!("Oh? And what does this braggart propose?"))
        .line(crew2!("Says he can outdrink and outnavigate any captain in port, Cap'n."))
        .line(crew3!("It's a challenge, clear as day. Lots of folks are watching. Could be good for our reputation if we win."))
        .line(captain!("Interesting... What are our options?"))
        .conditional_choice("Accept drinking contest", accept_drinking_contest, actions.get_item(Item::Gold) >= 50)
        .conditional_choice("Accept navigation challenge", accept_navigation_challenge, actions.get_item(Item::Gold) >= 75)
        .choice("Decline challenge", decline_challenge)
        .hint("Squawk! A wise captain knows when to compete and when to retreat!")
}