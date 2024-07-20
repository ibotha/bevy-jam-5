//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, ImageKey},
    screen::Screen,
};

use super::journey::CreateJourney;
use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,

    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.trigger(SpawnPlayer);
    commands.trigger(CreateJourney);
}
