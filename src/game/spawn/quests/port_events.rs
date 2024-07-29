use super::{port_stories::*, prelude::*};

pub(super) fn select_random_port_event(
    actions: &mut StoryActions,
    port: Port,
) -> super::EventBuilder {
    if actions.get_oocured_events().len() > 5 {
        return the_day_at_port_event;
    }
    let choices = [
        (the_day_at_port_event as EventBuilder, 14), // Higher number has  a higher chance of being selected
        // Common events
        (the_dockside_market_event, 8),
        (the_harbor_gossip_event, 8),
        (the_local_tavern_visit_event, 8),
        (the_harbor_maintenance_day_event, 8),
        // Uncommon events
        (the_dockworkers_dispute_event, 6),
        (the_foreign_diplomats_request_event, 6),
        (the_carnival_in_the_city_event, 6),
        (the_merchants_dilemma_event, 6),
        // Rare events
        (the_mysterious_cartographer_event, 4),
        (the_smugglers_offer_event, 4),
        (the_harbormasters_dilemma_event, 4),
        (the_rival_captains_challenge_event, 4),
        (the_smugglers_auction_event, 4),
        (the_dockside_heist_event, 4),
        (the_ports_grand_regatta_event, 4),
        // Very rare events
        //(the_mysterious_shipwreck_event, 3),
        (the_cursed_cargo_event, 3),
        (the_sirens_song_festival_event, 3),
        (the_cursed_lighthouse_event, 3),
        //(the_haunted_shipwreck_event, 3),
        // Special events
        (the_ancient_pirate_kings_challenge_event, 2),
        //(the_lost_city_of_atlantis_resurfaces_event, 2),
        // legendary events
        (the_legendary_sea_monster_sighting_event, 1),
    ];

    // Filter out any journeys that are in occured.
    // Under Journey.get_oocured_events()
    let occurred_events = actions.get_oocured_events();

    let available_choices: Vec<_> = choices
        .iter()
        .filter(|(event, _)| !occurred_events.contains(event))
        .cloned() // Clone the tuple since we're working with references
        .collect();

    let choices_to_use = if available_choices.is_empty() {
        &choices[..]
    } else {
        &available_choices[..]
    };

    if available_choices.len() > 5 {
        return the_day_at_port_event;
    }
    weighted_random(Some(actions.get_journey_rng()), &choices_to_use).clone()
}
