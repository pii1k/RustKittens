use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct ShowLine {
    pub text: String,
}

#[derive(Event, Debug, Clone)]
pub struct ShowChoices {
    pub items: Vec<String>,
}

#[derive(Event, Debug, Clone)]
pub struct DialogEnded;