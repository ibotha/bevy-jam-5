//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::game::ui::SpawnGameUI;

use super::journey::CreateJourney;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnGameUI);
    commands.trigger(CreateJourney);
}
