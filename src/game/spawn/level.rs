//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::game::ui::{Focus, FocusedDisplay, SpawnGameUI, UpdateShipStatsUI};

use super::{
    journey::CreateJourney,
    predicitons::{UpdateDarkMagicUi, UpdateParrotUi, UpdateSpyGlassUi},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnGameUI);
    commands.trigger(CreateJourney);
    commands.trigger(Focus(FocusedDisplay::Dialogue));

    commands.trigger(UpdateDarkMagicUi);
    commands.trigger(UpdateShipStatsUI);
    commands.trigger(UpdateParrotUi);
    commands.trigger(UpdateSpyGlassUi);
}

