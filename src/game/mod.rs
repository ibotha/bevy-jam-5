//! Game mechanics and content.

use bevy::prelude::*;
use rand::{Rng, RngCore};

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;
pub mod ui;

pub fn weighted_random<'a, T, R: RngCore>(rng: Option<&mut R>, iter: &'a [(T, u32)]) -> &'a T {
    let total: u32 = iter.iter().map(|(_, b)| b).sum();
    let mut gen = match rng {
        Some(rng) => rng.gen_range(0..total),
        None => rand::thread_rng().gen_range(0..total),
    };
    for (item, ballots) in iter.iter() {
        if gen < *ballots {
            return item;
        }
        gen -= ballots;
    }
    info!("We should never get here, the ballots should be used up");
    panic!("Panicked when generating random value from ballots!")
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        ui::plugin,
    ));
}
