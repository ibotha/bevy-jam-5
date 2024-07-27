use std::sync::atomic::{AtomicUsize, Ordering};

use bevy::utils::HashMap;

use super::{dialogue::Dialogue, ChoiceFunction};
static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
#[derive(Debug, Clone)]
pub struct DayEvent {
    id: usize,
    pub dialog: Vec<Dialogue>,
    pub choices: HashMap<String, ChoiceFunction>,
    pub hint_string: Option<String>,
}

impl PartialEq for DayEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DayEvent {
    pub fn new() -> Self {
        Self {
            dialog: vec![],
            choices: HashMap::new(),
            id: get_id(),
            hint_string: None,
        }
    }

    pub fn line(mut self, d: Dialogue) -> Self {
        self.dialog.push(d);
        self
    }

    pub fn choice<T: ToString>(mut self, name: T, action: ChoiceFunction) -> Self {
        self.choices.insert(name.to_string(), action);
        self
    }

    pub fn conditional_choice<T: ToString>(
        mut self,
        name: T,
        action: ChoiceFunction,
        condition: bool,
    ) -> Self {
        if condition {
            self.choices.insert(name.to_string(), action);
        }
        self
    }

    pub fn hint<T: ToString>(mut self, hint: T) -> Self {
        self.hint_string = Some(hint.to_string());
        self
    }
}
