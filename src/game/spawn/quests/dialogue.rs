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
    pub fn new(speaker: &str) -> Self {
        Self {
            speaker: speaker.to_owned(),
            paragraphs: vec![],
        }
    }

    pub fn para<T: ToString>(mut self, str: T) -> Self {
        self.paragraphs.push(str.to_string());
        self
    }

    pub(crate) fn paras(mut self, strings: &[String]) -> Self {
        for s in strings.iter() {
            self.paragraphs.push(s.clone());
        }
        self
    }
}
