//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod journey;
pub mod journey_constants;
pub mod level;
pub mod player;
pub mod quests;
pub mod weather;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, journey::plugin));
}
