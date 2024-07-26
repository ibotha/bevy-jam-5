use core::fmt;
use std::{f32::consts::PI, fmt::Formatter};

use bevy::prelude::*;
use rand::{rngs::StdRng, Rng};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum Moisture {
    Dry,
    Comfortable,
    Humid,
}

impl Default for Moisture {
    fn default() -> Self {
        Self::Comfortable
    }
}

impl Moisture {
    /// Generate a random moisture value based off of a cycle.
    /// `intensity` relates to how extreme the cycles are, 10 is the baseline
    /// `cycle_length` Is the number of days needed for a complete cycle
    /// `current_day`\ where are we now in the cycle
    pub(super) fn generate_from_cycle(
        rng: &mut StdRng,
        intensity: f32,
        cycle_length: u32,
        current_day: u32,
    ) -> Self {
        // The dryness is based off of a sine wave, cycling backward and forward
        let dryness = ((2.0 * current_day as f32 * PI) / cycle_length as f32).sin();
        let dry_ballots = ((intensity * dryness) as u32).max(0);
        let comfortable_ballots = 10;
        let humid_ballots = ((intensity * -dryness) as u32).max(0);

        let total_ballots = dry_ballots + comfortable_ballots + humid_ballots;
        let ballot = rng.gen_range(0..total_ballots);
        if ballot < dry_ballots {
            return Moisture::Dry;
        }
        if ballot < (dry_ballots + comfortable_ballots) {
            return Moisture::Comfortable;
        }
        Moisture::Humid
    }
}

#[cfg(test)]
mod tests {
    use log::info;
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn test_moisture_cycle() {
        env_logger::init();
        let mut rng = StdRng::seed_from_u64(695);
        for i in 0..80 {
            let moisture = Moisture::generate_from_cycle(&mut rng, i as f32, 40, i);
            let heat = Heat::generate_from_cycle(&mut rng, i as f32, 80, i);
            let wind = Wind::generate_from_cycle(&mut rng, i as f32, 20, i);
            info!(
                "Day {i:3}: ({m:<12}|{h:<12}|{w:<12})",
                m = format!("{moisture:?}"),
                h = format!("{heat:?}"),
                w = format!("{wind:?}")
            );
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum Heat {
    Blistering,
    Warm,
    Comfortable,
    Chilly,
    Freezing,
}

impl Default for Heat {
    fn default() -> Self {
        Self::Comfortable
    }
}

impl Heat {
    pub(super) fn generate_from_cycle(
        rng: &mut StdRng,
        intensity: f32,
        cycle_length: u32,
        current_day: u32,
    ) -> Self {
        let heat = ((2.0 * current_day as f32 * PI) / cycle_length as f32).sin() * intensity;
        let mid_likelyhood = 0.7 * heat;
        let extreme_likelyhood = 0.3 * heat;
        let blistering_ballots = (extreme_likelyhood as u32).max(0);
        let warm_ballots = (mid_likelyhood as u32).max(0);
        let comfortable_ballots = 10;
        let chilly_ballots = (-mid_likelyhood as u32).max(0);
        let freezing_ballots = (-extreme_likelyhood as u32).max(0);

        let total_ballots = blistering_ballots
            + warm_ballots
            + comfortable_ballots
            + chilly_ballots
            + freezing_ballots;
        let ballot = rng.gen_range(0..total_ballots);
        if ballot < blistering_ballots {
            return Heat::Blistering;
        }
        if ballot < (blistering_ballots + warm_ballots) {
            return Heat::Warm;
        }
        if ballot < (blistering_ballots + warm_ballots + comfortable_ballots) {
            return Heat::Comfortable;
        }
        if ballot < (blistering_ballots + warm_ballots + comfortable_ballots + chilly_ballots) {
            return Heat::Chilly;
        }
        Heat::Freezing
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum Wind {
    None,
    Low,
    Medium,
    High,
    GaleForce,
}

impl Default for Wind {
    fn default() -> Self {
        Self::Medium
    }
}

impl Wind {
    pub(super) fn generate_from_cycle(
        rng: &mut StdRng,
        intensity: f32,
        cycle_length: u32,
        current_day: u32,
    ) -> Self {
        let heat = ((2.0 * current_day as f32 * PI) / cycle_length as f32).sin() * intensity;
        let mid_likelyhood = 0.7 * heat;
        let extreme_likelyhood = 0.3 * heat;
        let none_ballots = (extreme_likelyhood as u32).max(0);
        let low_ballots = (mid_likelyhood as u32).max(0);
        let medium_ballots = 10;
        let high_ballots = (-mid_likelyhood as u32).max(0);
        let galeforce_ballots = (-extreme_likelyhood as u32).max(0);

        let total_ballots =
            none_ballots + low_ballots + medium_ballots + high_ballots + galeforce_ballots;
        let ballot = rng.gen_range(0..total_ballots);
        if ballot < none_ballots {
            return Wind::None;
        }
        if ballot < (none_ballots + low_ballots) {
            return Wind::Low;
        }
        if ballot < (none_ballots + low_ballots + medium_ballots) {
            return Wind::Medium;
        }
        if ballot < (none_ballots + low_ballots + medium_ballots + high_ballots) {
            return Wind::High;
        }
        Wind::GaleForce
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum AnyWeather {
    Heat(Heat),
    Moisture(Moisture),
    Wind(Wind),
}

impl fmt::Display for Heat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Moisture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct DayWeather {
    pub wind: Wind,
    pub heat: Heat,
    pub moisture: Moisture,
}
