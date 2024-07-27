use rand::rngs::{StdRng, ThreadRng};

use crate::game::spawn::{
    journey::{Journey, Ship},
    weather::DayWeather,
};

use super::{dialogue::DialogueQueue, prelude::*};

pub struct StoryActions<'a> {
    ship: &'a mut Ship,
    journey: &'a mut Journey,
    dialogue: &'a mut DialogueQueue,
    updates: &'a mut Vec<String>,
}

fn diff_readout(diff: i32, unit: &str, pluralize: bool) -> String {
    base_diff_readout(diff, ("gained", "lost"), unit, pluralize)
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

    pub fn once_off(&mut self, name: &'static str) -> bool {
        return self.journey.once_offs.insert(name);
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

        if let Environment::Sea(_) = env {
            self.delta_crew(self.ship.left_behind);
            self.ship.left_behind = 0;
        }

        self.journey.environment = env;
        self.updates.push(match env {
            Environment::Port(p) => format!(
                "You arrive at {port}",
                port = match p {
                    Port::Random => "port",
                    Port::Any => "",
                    Port::Tortuga => "Tortuga",
                    Port::ShadyCove => "Shady Cove",
                    Port::EdgeOfTheWorld => "The Edge of The World",
                    Port::RoyalNavyBase => "The Royal Navy Base",
                }
            ),
            Environment::Island(i) => format!(
                "You set foot on {island}",
                island = match i {
                    Island::Random => "the island",
                    Island::Any => "",
                    Island::MysteriousIsland => "the mysterious island",
                    Island::SirensCove => "sirens cove",
                    Island::TrinketSeller => "goldfang island",
                }
            ),
            Environment::Sea(s) => format!(
                "You cast off into {sea}",
                sea = match s {
                    Sea::Intro => "sea",
                    Sea::Any => todo!(),
                    Sea::Northern => "northern sea",
                    Sea::SirensCove => "singing seas",
                }
            ),
        });
    }

    pub fn travel(&mut self, distance: i32) {
        self.updates
            .push(format!("You covered {distance} leagues.",));
        self.journey.distance += distance;
        for event in &mut self.journey.events {
            if event.environment == self.journey.environment {
                if let Delay::Distance(d) = &mut event.delay {
                    *d -= distance;
                }
            }
        }
    }

    pub(crate) fn weather(&self) -> DayWeather {
        self.journey.weather.clone()
    }

    pub(crate) fn get_current_sea(&self) -> Sea {
        self.journey.sea
    }

    pub(crate) fn get_clarity(&self) -> i32 {
        let DW {
            heat,
            moisture,
            wind,
        } = self.weather();

        let wind_factor = match wind {
            W::None => 0,
            W::Low => 0,
            W::Medium => 1,
            W::High => 2,
            W::GaleForce => 3,
        };
        match (heat, moisture) {
            (H::Blistering, M::Dry) => 10,
            (H::Blistering, M::Comfortable) => 8,
            (H::Blistering, M::Humid) => 7 - wind_factor,
            (H::Warm, M::Dry) => 10,
            (H::Warm, M::Comfortable) => 8,
            (H::Warm, M::Humid) => 6 - wind_factor,
            (H::Comfortable, M::Dry) => 10,
            (H::Comfortable, M::Comfortable) => 8,
            (H::Comfortable, M::Humid) => 5 - wind_factor,
            (H::Chilly, M::Dry) => 7,
            (H::Chilly, M::Comfortable) => 4,
            (H::Chilly, M::Humid) => 3 - wind_factor,
            (H::Freezing, M::Dry) => 6,
            (H::Freezing, M::Comfortable) => 4,
            (H::Freezing, M::Humid) => 1 - wind_factor,
        }
    }

    pub(crate) fn possible_distance(&self) -> i32 {
        let DW {
            heat,
            moisture,
            wind,
        } = self.weather();
        match self.get_environment() {
            Environment::Port(_) => 0,
            Environment::Island(_) => {
                6 - match wind {
                    W::None | W::Low | W::Medium => 0,
                    W::High => 1,
                    W::GaleForce => 2,
                } - match moisture {
                    M::Dry | M::Humid => 1,
                    _ => 0,
                } - match heat {
                    H::Blistering | H::Freezing => 2,
                    H::Warm | H::Chilly => 1,
                    _ => 0,
                }
            }
            Environment::Sea(_) => match wind {
                W::None => 0,
                W::Low => 2,
                W::Medium => 4,
                W::High => 8,
                W::GaleForce => 12,
            },
        }
    }

    pub(crate) fn danger(&self) -> i32 {
        let DW {
            heat,
            moisture,
            wind,
        } = self.weather();
        match self.get_environment() {
            Environment::Port(_) => 0,
            Environment::Island(_) => {
                0 + match wind {
                    W::None | W::Low | W::Medium => 0,
                    W::High => 1,
                    W::GaleForce => 2,
                } + match moisture {
                    M::Dry | M::Humid => 1,
                    _ => 0,
                } + match heat {
                    H::Blistering | H::Freezing => 3,
                    H::Warm | H::Chilly => 1,
                    _ => 0,
                }
            }
            Environment::Sea(_) => self.possible_distance().min(match (heat, moisture) {
                (H::Blistering, M::Dry) => 8,
                (H::Blistering, M::Comfortable) => 6,
                (H::Blistering, M::Humid) => 10,
                (H::Warm, M::Dry) => 5,
                (H::Warm, M::Comfortable) => 4,
                (H::Warm, M::Humid) => 6,
                (H::Comfortable, M::Dry) => 2,
                (H::Comfortable, M::Comfortable) => 0,
                (H::Comfortable, M::Humid) => 3,
                (H::Chilly, M::Dry) => 5,
                (H::Chilly, M::Comfortable) => 4,
                (H::Chilly, M::Humid) => 6,
                (H::Freezing, M::Dry) => 7,
                (H::Freezing, M::Comfortable) => 6,
                (H::Freezing, M::Humid) => 10,
            }),
        }
    }

    pub(crate) fn island_crew(&mut self, size: i32) {
        self.ship.left_behind = (self.ship.crew - size).max(0);
        self.delta_crew(-self.ship.left_behind);
    }
}
