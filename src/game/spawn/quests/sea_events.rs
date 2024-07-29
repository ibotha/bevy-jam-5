use super::{prelude::*, sea_stories::*};

pub fn set_next_port(actions: &mut StoryActions, distance: u32, sea: Option<Sea>) {
    let mut new_sea = actions.get_current_sea();
    if let Some(s) = sea {
        new_sea = s;
    }
    actions.add_event(FollowingEvent {
        event: port_spotted,
        certainty: Certainty::Possible(10),
        delay: Delay::Distance(distance as i32),
        environment: Environment::Sea(new_sea),
    });
}

fn dock(actions: &mut StoryActions) {
    actions.change_environment(super::Environment::Port(Port::Random));
}

fn sail_on(actions: &mut StoryActions) {
    sail(actions);
    set_next_port(actions, 20, None);
}

pub fn port_spotted(_actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(crew1!("Land HO! There is a port on the horizon."))
        .choice("Sail On", sail_on)
        .choice("Dock", dock)
}

fn explore_island(actions: &mut StoryActions) {
    actions.change_environment(super::Environment::Island(Island::Random));
}

fn island_spotted(actions: &mut StoryActions) -> DayEvent {
    sea_stories_base(actions)
        .line(crew1!("I see an island captain."))
        .choice("Explore", explore_island)
}

pub(super) fn select_random_sea_event(actions: &mut StoryActions, sea: Sea) -> EventBuilder {
    let choices = [
        (the_plain_sailing_event as EventBuilder, 50),
        // Common events
        (the_fishing_competition_event, 14),
        (the_mysterious_fog_event, 14),
        (the_floating_debris_event, 14),
        (the_albatross_omen_event, 14),
        // Uncommon events
        (the_ghost_ship_encounter_event, 10),
        (the_merfolk_trade_proposal_event, 10),
        //(the_kraken_s_bargain_event, 10),
        //(the_bermuda_triangle_anomaly_event, 10),
        // Rare events
        //(the_atlantis_resurfacing_event, 4),
        (the_flying_dutchman_challenge_event, 4),
        //(the_poseidon_s_trident_event, 4),
        //(the_kraken_s_lair_discovery_event, 4),
        (the_underwater_volcano_eruption_event, 4),
        (the_phantom_pirate_fleet_event, 4),
        //(the_time_rift_anomaly_event, 4),
        // Very rare events
        (the_celestial_alignment_event, 3),
        (the_atlantean_portal_discovery_event, 3),
        //(the_maelstrom_of_realities_event, 3),
        (the_cosmic_kraken_encounter_event, 3),
        //(the_bermuda_triangle_time_warp_event, 3),
        // Special events
        (the_leviathan_awakening_event, 2),
        (the_convergence_of_realms_event, 2),
        // legendary events
        (the_celestial_sea_parting_event, 1),
        (island_spotted, 1),
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

    weighted_random(Some(actions.get_journey_rng()), &choices_to_use).clone()
}
