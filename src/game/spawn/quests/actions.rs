use rand::rngs::{StdRng, ThreadRng};

use crate::game::spawn::{
    journey::{Journey, Ship},
    weather::DayWeather,
};

use super::{
    dialogue::{Dialogue, DialogueQueue},
    treasure::Item,
    Environment, FollowingEvent,
};

pub struct StoryActions<'a> {
    ship: &'a mut Ship,
    journey: &'a mut Journey,
    dialogue: &'a mut DialogueQueue,
    updates: &'a mut Vec<String>,
}

fn diff_readout(diff: i32, unit: &str, pluralize: bool) -> String {
    base_diff_readout(diff, ("gained", "lossed"), unit, pluralize)
}

fn damage_diff_readout(diff: i32) -> String {
    base_diff_readout(diff, ("restored", "took"), "damage", false)
}

fn base_diff_readout(diff: i32, verbs: (&str, &str), unit: &str, pluralize: bool) -> String {
    format!(
        "You {verb} {abs_diff} {unit}{pluralize}",
        verb = if diff > 0 { verbs.0 } else { verbs.1 },
        pluralize = if pluralize && (diff != 1) { "s" } else { "" },
        abs_diff = diff.abs()
    )
}

impl<'a> StoryActions<'a> {
    pub fn new(
        ship: &'a mut Ship,
        journey: &'a mut Journey,
        dialogue: &'a mut DialogueQueue,
        updates: &'a mut Vec<String>,
    ) -> Self {
        Self {
            ship,
            journey,
            dialogue,
            updates,
        }
    }
    pub fn delta_crew(&mut self, arg: i32) {
        let arg = arg.min(self.ship.max_crew - self.ship.crew);
        if arg == 0 {
            return;
        }
        self.updates.push(diff_readout(arg, "crew member", true));

        self.ship.crew += arg;
    }

    pub fn delta_max_crew(&mut self, arg: i32) {
        let arg = arg.max(-self.ship.max_crew);
        if arg == 0 {
            return;
        }
        self.updates
            .push(diff_readout(arg, "crew member capacity", false));
        self.ship.max_crew += arg;
    }

    pub fn get_crew(&self) -> i32 {
        self.ship.crew
    }

    pub fn delta_food(&mut self, arg: i32) {
        let arg = arg.min(self.ship.max_food - self.ship.food);
        if arg == 0 {
            return;
        }
        self.updates.push(diff_readout(arg, "food", false));
        self.ship.food += arg;
    }

    pub fn delta_max_food(&mut self, arg: i32) {
        let arg = arg.max(-self.ship.max_food);
        if arg == 0 {
            return;
        }
        self.updates.push(diff_readout(arg, "food capacity", false));
        self.ship.max_food += arg;
    }

    pub fn delta_health(&mut self, arg: i32) {
        let arg = arg.min(self.ship.max_health - self.ship.health);
        if arg == 0 {
            return;
        }
        self.updates.push(damage_diff_readout(arg));
        self.ship.health += arg;
    }

    pub fn delta_max_health(&mut self, arg: i32) {
        let arg = arg.max(-self.ship.max_health);
        if arg == 0 {
            return;
        }
        self.updates
            .push(diff_readout(arg, "max ship health", false));
        self.ship.max_health += arg;
    }

    pub fn add_event(&mut self, event: FollowingEvent) {
        self.journey.events.push(event);
    }

    pub fn get_item(&self, item: Item) -> i32 {
        *self.journey.inventory.get(&item).unwrap_or(&0)
    }

    pub fn delta_items(&mut self, item: Item, delta: i32) {
        let current = self.get_item(item);

        let delta = delta.max(-current);
        if delta == 0 {
            return;
        }
        self.updates
            .push(diff_readout(delta, format!("{item}").as_str(), true));
        self.journey.inventory.insert(item, current + delta);
    }

    pub fn add_dialogue(&mut self, dialogue: Dialogue) {
        self.dialogue.queue.push_back(dialogue);
    }

    pub fn get_environment(&self) -> Environment {
        self.journey.environment
    }

    pub fn get_journey_rng(&mut self) -> &mut StdRng {
        &mut self.journey.rng
    }

    pub fn get_rng(&mut self) -> ThreadRng {
        rand::thread_rng()
    }

    pub fn change_environment(&mut self, env: Environment) {
        if self.journey.environment == env {
            return;
        }
        self.journey.environment = env;
        self.updates.push(match env {
            Environment::Port => "You arrive at port".to_string(),
            Environment::Island => "You set foot on the island".to_string(),
            Environment::Sea => "You cast off into the sea".to_string(),
        });
    }

    pub fn travel(&mut self, distance: i32) {
        self.updates
            .push(format!("You covered {distance} leagues.",));
        self.journey.distance += distance;
        for event in &mut self.journey.events {
            event.distance -= distance;
        }
    }

    pub(crate) fn weather(&self) -> DayWeather {
        self.journey.weather.clone()
    }
}
