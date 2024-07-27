use super::prelude::*;
/// Engage in battle with the enemy. `strength` is how strong the enemy is 1 is weak 10 is very
/// strong
pub fn battle(actions: &mut StoryActions, enemy_strength: i32, strength_mod: i32, name: &str) {
    let our_strength = actions.get_item(Item::Cannon).min(actions.get_crew()) + strength_mod;

    // Higher numbers mean enemy is stronger
    let battle_favour = enemy_strength - our_strength;

    let damage = battle_favour.max(0);
    actions.delta_health(-damage);
}
