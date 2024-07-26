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
    pub hint: Option<String>,
}

impl PartialEq for DayEvent {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl DayEvent {
    /// Create a new event with dialogue options and choices
    ///
    /// Here is and example:
    /// ```rust
    /// fn embark(ship: Ship, weather: &DayWeather) -> ChoiceResult {
    ///     ChoiceResult {
    ///         ship,
    ///         following_events: vec![
    ///             FollowingEvent {
    ///                 event: visit_shady_cove(),
    ///                 certainty: Certainty::Certain,
    ///                 distance: 20,
    ///                 environment: Environment::Sea
    ///             }
    ///         ],
    ///     }
    /// }
    ///
    /// pub fn embark_event() -> DayEvent {
    ///     DayEvent::new(
    ///         &[
    ///             Dialogue::new(CAPTAIN, &["We are headed to get the trident!!!"]),
    ///             Dialogue::new(CREW_MEMBER, &["What?!?!? The trident!!!"]),
    ///             Dialogue::new(CAPTAIN, &["Yes! The trident"]),
    ///         ],
    ///         &[("Embark!", embark)],
    ///     )
    /// }
    /// ```
    pub fn new(dialogue: &[Dialogue], choices: &[(&str, ChoiceFunction)]) -> Self {
        Self {
            dialog: dialogue.iter().map(|d| d.to_owned()).collect(),
            choices: HashMap::from_iter(
                choices
                    .iter()
                    .map(|(label, method)| (label.to_string(), *method)),
            ),
            id: get_id(),
            hint: None,
        }
    }

    pub fn add_hints<T: ToString>(mut self, hint: T) -> Self {
        self.hint = Some(hint.to_string());
        return self;
    }
}
