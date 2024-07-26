//! Game mechanics and content.

use bevy::prelude::*;
use rand::Rng;

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;
mod ui;

pub fn weighted_random<T, R>(rng: Option<&mut R>, iter: &[(T, u32)]) -> T
where
    R: Rng,
    T: Clone,
{
    let total: u32 = iter.iter().map(|(_, b)| b).sum();
    let mut gen = match rng {
        Some(rng) => rng.gen_range(0..total),
        None => rand::thread_rng().gen_range(0..total),
    };
    for (item, ballots) in iter {
        if gen < *ballots {
            return item.clone();
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
