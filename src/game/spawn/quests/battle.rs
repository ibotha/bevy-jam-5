use super::prelude::*;

impl StoryActions<'_> {
    /// Engage in battle with the enemy. `strength` is how strong the enemy is 1 is weak 10 is very
    /// strong.
    ///
    /// Returns a number representing the battle favour, negative numbers mean the enemy is winning more.
    /// positive numbers mean we are winning;
    pub fn battle(&mut self, enemy_strength: i32, strength_mod: i32, name: &str) -> i32 {
        let our_strength = self.get_item(Item::Cannon).min(self.get_crew()) + strength_mod;

        // lower numbers mean enemy is stronger
        let battle_favour = our_strength - enemy_strength;
        self.add_dialogue(dialogue!(
        "Battle";
        format!("You fight {name}!").as_str(),
        format!("The battle favours {degree}.", degree=battle_degree(battle_favour))
        ));

        let damage = (-battle_favour).max(0);
        self.delta_health(-damage);
        self.delta_crew(-damage / 4);
        battle_favour
    }
}

fn battle_degree(favour: i32) -> &'static str {
    if favour < -10 {
        "the enemy gravely"
    } else if favour < -5 {
        "the enemy"
    } else if favour < 0 {
        "the enemy slightly"
    } else if favour == 0 {
        "no one"
    } else if favour < 5 {
        "you slightly"
    } else if favour < 10 {
        "you"
    } else {
        "you greatly"
    }
}
