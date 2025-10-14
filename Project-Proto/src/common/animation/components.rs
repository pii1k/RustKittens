use std::hash::Hash;

use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct AnimationController<S: AnimationState> {
    pub state: S,
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub direction: Direction8,
}

impl<S: AnimationState> AnimationController<S> {
    pub fn new(initial_state: S, direction: Direction8) -> Self {
        Self {
            state: initial_state,
            direction,
            current_frame: 0,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
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

impl AnimationSet {
    pub fn new() -> Self {
        Self {
            clips: HashMap::default(),
        }
    }

    pub fn add_clip(&mut self, name: impl Into<String>, clip: AnimationClip) -> &mut Self {
        self.clips.insert(name.into(), clip);
        self
    }

    pub fn get_clip(&self, name: &str) -> Option<&AnimationClip> {
        self.clips.get(name)
    }
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

impl Direction8 {
    pub fn from_velocity(velocity: Vec2) -> Self {
        if velocity.length() < 0.01 {
            return Direction8::South;
        }

        let angle = velocity.y.atan2(velocity.x);
        let degrees = angle.to_degrees();

        let normalized = (degrees + 22.5) / 45.0;
        let idx = ((normalized.floor() as i32 + 8) % 8) as usize;

        match idx {
            0 => Self::East,      // 0°
            1 => Self::NorthEast, // 45°
            2 => Self::North,     // 90°
            3 => Self::NorthWest, // 135°
            4 => Self::West,      // 180°
            5 => Self::SouthWest, // 225°
            6 => Self::South,     // 270°
            7 => Self::SouthEast, // 315°
            _ => Self::South,
        }
    }
}
