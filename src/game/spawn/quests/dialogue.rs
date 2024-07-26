use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct DialogueQueue {
    pub queue: VecDeque<Dialogue>,
}

#[derive(Debug, Reflect, Clone)]
pub struct Dialogue {
    pub speaker: String,
    pub paragraphs: Vec<String>,
}

impl Dialogue {
    pub fn new(speaker: &str, paragraphs: &[&str]) -> Self {
        Self {
            speaker: speaker.to_owned(),
            paragraphs: paragraphs.iter().map(|p| p.to_string()).collect(),
        }
    }
    pub fn new_from_strings<T: IntoIterator<Item = String>>(speaker: &str, paragraphs: T) -> Self {
        Self {
            speaker: speaker.to_owned(),
            paragraphs: paragraphs.into_iter().collect(),
        }
    }
}
