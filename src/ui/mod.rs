//! Reusable UI widgets & theming.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod interaction;
pub mod palette;
mod widgets;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::{prelude::*, window::WindowResized};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
    app.add_systems(Update, resize_ui);
}

fn resize_ui(mut resize_reader: EventReader<WindowResized>, mut ui_scale: ResMut<UiScale>) {
    for event in resize_reader.read() {
        ui_scale.0 = event.height as f32 / 250.0;
        info!("Updated UI Scale to {scale}", scale = ui_scale.0);
    }
}
