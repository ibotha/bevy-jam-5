use super::island_stories_base;
use crate::game::spawn::quests::prelude::*;

fn join_carnival(actions: &mut StoryActions) {
    let DW {
        heat,
        wind,
        moisture,
    } = actions.weather();

    match (heat, wind, moisture) {
        (H::Comfortable, W::Low, M::Comfortable) => {
            actions.delta_items(Item::Gold, 100);
            actions.delta_health(5);
            actions.delta_crew(3);
            actions.add_dialogue(captain!("Yarr, what a spectacle! We won the grand prize in the treasure hunt, our crew triumphed in the drinking contest, and we've recruited some colorful characters. The ship's practically buzzing with festive energy!"));
        }
        (H::Warm, W::Medium, _) => {
            actions.delta_items(Item::Gold, 50);
            actions.delta_health(-10);
            actions.delta_crew(1);
            actions.add_dialogue(captain!("A day of ups and downs at the carnival. We won some, lost some, but came out ahead in gold. The crew's a bit worse for wear after the festivities, but we've got a new member with some... interesting skills."));
        }
        (H::Blistering, W::High, _) | (_, W::GaleForce, _) => {
            actions.delta_items(Item::Gold, -20);
            actions.delta_health(-10);
            actions.delta_crew(-1);
            actions.add_dialogue(captain!("Blimey, what a disaster! The weather turned the carnival into chaos. We lost gold in rigged games, one of our crew disappeared in a magic act gone wrong, and the ship took some damage from flying debris. Let's get out of here!"));
        }
        _ => {
            actions.delta_items(Item::Gold, 30);
            actions.delta_health(10);
            actions.delta_food(20);
            actions.add_dialogue(captain!("An amusing diversion, that carnival. We didn't win big, but we enjoyed ourselves and came out a bit richer. Plus, we've stocked up on some exotic foods that should keep the crew happy for a while."));
        }
    }
}

fn perform_in_carnival(actions: &mut StoryActions) {
    if actions.get_item(Item::Cannon) >= 1 {
        actions.delta_items(Item::Cannon, -1);
        actions.delta_items(Item::Gold, 150);
        actions.delta_crew(2);
        actions.add_dialogue(captain!("Our cannon juggling act was a smashing success! We've won a mountain of gold and attracted two skilled performers to our crew. Shame about the cannon, but the crowd certainly enjoyed the grand finale!"));
    } else {
        actions.delta_items(Item::Gold, 20);
        actions.delta_health(-5);
        actions.add_dialogue(captain!("Without our cannon for the act, we had to improvise. Our sea shanties and sword swallowing earned us some gold, but the crowd was a tough one. A few rotten tomatoes have left their mark on both ship and crew."));
    }
}

fn plunder_carnival(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, 80);
    actions.delta_health(-15);
    actions.delta_crew(-2);
    actions.add_dialogue(captain!("We struck when they least expected! Grabbed a hefty sum from the treasury, but it wasn't without cost. Lost two men in the chaos, and we'll need to lay low for a while. Still, the haul was worth it... wasn't it?"));
}

pub fn the_pirates_carnival_event(actions: &mut StoryActions) -> DayEvent {
    island_stories_base(actions)
        .line(crew1!("Cap'n! There's a carnival ahead, and it's full of pirates!"))
        .line(captain!("A pirate's carnival? Now that's something you don't see every day."))
        .line(crew2!("Aye, Cap'n! Games of chance, feats of strength, and more grog than you can shake a peg leg at!"))
        .line(crew3!("We could put on a show of our own, Cap'n. Might win us some coin... or we could always take the direct approach."))
        .line(captain!("A carnival of scurvy dogs, eh? Sounds like trouble... and opportunity. What's our play, lads?"))
        .choice("Join Carnival", join_carnival)
        .conditional_choice("Perform Act", perform_in_carnival, actions.get_item(Item::Cannon) >= 1)
        .choice("Plunder", plunder_carnival)
        .hint("Squawk! Even pirates need a day off... or a day to make some extra booty!")
}