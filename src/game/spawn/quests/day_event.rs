use bevy::utils::HashMap;

use super::{dialogue::Dialogue, ChoiceFunction};

#[derive(Debug, Clone)]
pub struct DayEvent {
    pub dialog: Vec<Dialogue>,
    pub choices: HashMap<String, ChoiceFunction>,
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
        }
    }
}
