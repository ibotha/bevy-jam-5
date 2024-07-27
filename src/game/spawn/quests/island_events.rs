use super::{island_stories::*, prelude::*};

pub(super) fn select_random_island_event(
    actions: &mut StoryActions,
    island: Island,
) -> EventBuilder {
    let choices = [
        (the_just_walking_event as EventBuilder, 14),
        // Common events
        (the_hidden_cove_treasure_event, 8),
        (the_mysterious_shipwreck_event, 8),
        (the_tropical_fruit_grove_event, 8),
        (the_ancient_temple_ruins_event, 8),
        // Uncommon events
        (the_stranded_sailor_event, 6),
        (the_mysterious_fog_bank_event, 6),
        (the_abandoned_pirate_outpost_event, 6),
        (the_mysterious_idol_event, 6),
        // Rare events
        (the_lost_city_of_gold_event, 4),
        (the_krakens_treasure_event, 4),
        (the_bermuda_triangle_anomaly_event, 4),
        (the_fountain_of_youth_event, 4),
        (the_ghost_ships_challenge_event, 4),
        (the_pirate_kings_tomb_event, 4),
        (the_mermaids_grotto_event, 4),
        // Very rare events
        (island_stories_base, 3),
        (the_krakens_awakening_event, 3),
        (the_time_storm_event, 3),
        (the_celestial_island_event, 3),
        (the_void_maelstrom_event, 3),
        // Special events
        (the_pirates_carnival_event, 2),
        (the_pirates_regatta_event, 2),
        // legendary events
        (the_awakening_of_the_kraken_god_event, 1),
    ];
    weighted_random(Some(actions.get_journey_rng()), &choices).clone()
}
