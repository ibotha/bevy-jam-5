use super::{prelude::*, port_stories::*};

pub(super) fn select_random_port_event(actions: &mut StoryActions) -> DayEvent {
    let choices = [
        (the_day_at_port_event(actions), 14), // Higher number has  a higher chance of being selected

        // Common events
        (the_dockside_market_event(actions), 8),
        (the_harbor_gossip_event(actions), 8),
        (the_local_tavern_visit_event(actions), 8),
        (the_harbor_maintenance_day_event(actions), 8),

        // Uncommon events
        (the_dockworkers_dispute_event(actions), 6),
        (the_foreign_diplomats_request_event(actions), 6),
        (the_carnival_in_the_city_event(actions), 6),
        (the_merchants_dilemma_event(actions), 6),

        // Rare events
        (the_mysterious_cartographer_event(actions), 4),
        (the_smugglers_offer_event(actions), 4),
        (the_harbormasters_dilemma_event(actions), 4),
        (the_rival_captains_challenge_event(actions), 4),
        (the_smugglers_auction_event(actions), 4),
        (the_dockside_heist_event(actions), 4),
        (the_ports_grand_regatta_event(actions), 4),

        // Very rare events
        (the_mysterious_shipwreck_event(actions), 3),
        (the_cursed_cargo_event(actions), 3),
        (the_sirens_song_festival_event(actions), 3),
        (the_cursed_lighthouse_event(actions), 3),
        (the_haunted_shipwreck_event(actions), 3),

        // Special events
        (the_ancient_pirate_kings_challenge_event(actions), 2),
        (the_lost_city_of_atlantis_resurfaces_event(actions), 2),
        
        // legendary events
        (the_legendary_sea_monster_sighting_event(actions), 1),
    ];

    weighted_random(Some(actions.get_journey_rng()), &choices).clone()
}