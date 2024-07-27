use super::{
    port_stories::*,
    prelude::*,
    sea_events::{bounty_hunters, sail, set_next_port},
};

fn embark(actions: &mut StoryActions) {
    actions.change_environment(Environment::Sea);
    sail(actions);
    set_next_port(actions, 20);
}

fn recruit(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -100);
    actions.delta_crew(1);
}

fn resupply(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -20);
    actions.delta_food(50);
}

fn repair(actions: &mut StoryActions) {
    actions.delta_items(Item::Gold, -100);
    actions.delta_health(10);
}

fn wait(_actions: &mut StoryActions) {}

fn a_day_at_port(actions: &mut StoryActions) -> DayEvent {
    DayEvent::new()
        .line(captain!(
            "We are still at the blasted port!",
            "Is the weather right for us to embark?"
        ))
        .choice("Embark", embark)
        .choice("Wait", wait)
        .conditional_choice("Recruit", recruit, actions.get_item(Item::Gold) > 100)
        .conditional_choice("Resupply", resupply, actions.get_item(Item::Gold) > 20)
        .conditional_choice("Repair", repair, actions.get_item(Item::Gold) > 100)
}

pub(super) fn select_random_port_event(actions: &mut StoryActions) -> super::EventBuilder {
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
        (the_mysterious_shipwreck_event, 3),
        (the_cursed_cargo_event, 3),
        (the_sirens_song_festival_event, 3),
        (the_cursed_lighthouse_event, 3),
        (the_haunted_shipwreck_event, 3),
        // Special events
        (the_ancient_pirate_kings_challenge_event, 2),
        (the_lost_city_of_atlantis_resurfaces_event, 2),
        // legendary events
        (the_legendary_sea_monster_sighting_event, 1),
    ];

    weighted_random(Some(actions.get_journey_rng()), &choices).clone()
}

