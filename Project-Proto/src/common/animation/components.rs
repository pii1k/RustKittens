use std::hash::Hash;

use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct AnimationController<S: AnimationState> {
    pub state: S,
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub direction: Direction8,
}

#[derive(Clone)]
pub struct AnimationClip {
    pub image_handle: Handle<Image>,
    pub texture_layout_handle: Handle<TextureAtlasLayout>,
    pub frames_per_direction: usize,
    pub frame_duration: f32,
    pub looping: bool,
}

#[derive(Component, Clone)]
pub struct AnimationSet {
    pub clips: HashMap<String, AnimationClip>,
}

pub trait AnimationState: Component + Clone + PartialEq + Eq + Hash {
    fn clip_name(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction8 {
    East = 0,
    SouthEast = 1,
    South = 2,
    SouthWest = 3,
    West = 4,
    NorthWest = 5,
    North = 6,
    NorthEast = 7,
}
