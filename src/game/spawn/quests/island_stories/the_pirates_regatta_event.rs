use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn enter_race(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (wind, moisture) {
        (W::High, M::Dry) => {
            actions.delta_items(Item::Gold, 3000);
            actions.delta_health(40);
            actions.delta_crew(4);
            actions.add_dialogue(captain!("Huzzah! We've won the Pirate's Regatta! Our ship cut through the waves like a hot knife through butter. The prize money is hefty, our crew's morale is sky-high, and we've attracted some skilled sailors to our ranks. We're the talk of the seven seas!"));
        }
        (W::Medium, M::Comfortable) => {
            actions.delta_items(Item::Gold, 1500);
            actions.delta_health(20);
            actions.delta_items(Item::Cannon, 1);
            actions.add_dialogue(captain!("A respectable second place! The prize money's nothing to scoff at, and we've won a fancy new cannon to boot. The crew's in high spirits, and our ship seems to have found a new spring in its step... er, sail."));
        }
        (W::Low, _) | (W::None, _) => {
            actions.delta_items(Item::Gold, -500);
            actions.delta_health(-10);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("Blimey, what a disappointment! The lack of wind left us floundering. We lost our entry fee, the crew's morale has taken a hit, and our navigator jumped ship in shame. At least we learned a few tricks from watching the winners."));
        }
        _ => {
            actions.delta_items(Item::Gold, 500);
            actions.delta_health(10);
            actions.delta_food(30);
            actions.add_dialogue(captain!("Middle of the pack, but not a bad showing! We've won back our entry fee with a bit extra, and scored an invitation to the after-party. The crew's enjoying the free food and drink, and we've picked up some useful sailing tips."));
        }
    }
}

fn sabotage_competitors(actions: &mut StoryActions) {
    if actions.get_crew() >= 15 {
        actions.delta_items(Item::Gold, 4000);
        actions.delta_items(Item::Cannon, 2);
        actions.delta_crew(-1);
        actions.delta_health(-20);
        actions.add_dialogue(captain!("Our dastardly plan worked perfectly... almost. We've sabotaged the top contenders and claimed first prize by default! The gold is ours, along with two exceptional cannons. We lost one man in the process and the ship took some damage in our hasty escape, but the legends of our cunning will live on!"));
    } else {
        actions.delta_items(Item::Gold, -1000);
        actions.delta_health(-30);
        actions.delta_crew(-2);
        actions.add_dialogue(captain!("Curse our luck! We didn't have enough hands to pull off the sabotage. Got caught red-handed, lost two good men in the scuffle, and had to pay a hefty fine. Our reputation's taken a hit, and we're banned from future regattas. Back to honest pirating, I suppose."));
    }
}

fn host_side_bets(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 2500);
    actions.delta_crew(2);
    actions.delta_food(-20);
    actions.add_dialogue(captain!("Who needs to race when you can play the odds? Our betting ring was a smashing success! We've made a tidy profit, and two savvy gamblers have decided to join our crew. We had to use some of our supplies to host the event, but it was worth it for the connections we've made."));
}

pub fn the_pirates_regatta_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a grand commotion in the harbor - it's the annual Pirate's Regatta!"))
        .line(captain!("The Regatta, you say? Now that's an opportunity if I ever saw one."))
        .line(crew2!("Aye, Cap'n! Ships from all over are competing. The prize is a mountain of gold and eternal glory!"))
        .line(crew3!("We could race, sure, but there might be... other ways to profit from this event, if you catch my drift."))
        .line(captain!("A test of our sailing skills, our cunning, or our luck. What say you, crew? How shall we tackle this Regatta?"))
        .choice("Enter Race", enter_race)
        .conditional_choice("Sabotage", sabotage_competitors, actions.get_crew() >= 15)
        .choice("Host Bets", host_side_bets)
        .hint("Squawk! In a pirate's race, there's more than one way to cross the finish line!")
}