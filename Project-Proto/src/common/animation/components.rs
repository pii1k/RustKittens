use std::hash::Hash;

use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct AnimationController<S: AnimationState> {
    pub state: S,
    pub frame_timer: Timer,
    pub current_frame_idx: usize,
    pub direction: Direction8,
    pub is_finished: bool,
}

impl<S: AnimationState> AnimationController<S> {
    pub fn new(initial_state: S, direction: Direction8) -> Self {
        Self {
            state: initial_state,
            direction,
            current_frame_idx: 0,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            is_finished: false,
        }
    }

    pub fn is_animation_finished(&self) -> bool {
        self.is_finished
    }

    pub fn change_state(&mut self, new_state: S) {
        if self.state != new_state {
            self.state = new_state;
            self.current_frame_idx = 0;
            self.is_finished = false;
            self.frame_timer.reset();
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

pub type AnimationChangedQuery<'w, 's, S> = Query<
    'w,
    's,
    (
        &'static AnimationSet,
        &'static mut AnimationController<S>,
        &'static mut Sprite,
    ),
    Changed<AnimationController<S>>,
>;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LowerBodyState {
    Idle,
    Walk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpperBodyState {
    Normal,
    Attack,
}

impl UpperBodyState {
    pub fn is_non_looping(&self) -> bool {
        matches!(self, Self::Attack)
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct AnimationLayers {
    pub lower_body: LowerBodyState,
    pub upper_body: UpperBodyState,
}

impl AnimationLayers {
    pub fn new(lower: LowerBodyState, upper: UpperBodyState) -> Self {
        Self {
            lower_body: lower,
            upper_body: upper,
        }
    }
}

pub trait AnimationState: Component + PartialEq {
    fn clip_name(&self) -> &str;
}

impl AnimationState for AnimationLayers {
    fn clip_name(&self) -> &str {
        match (&self.lower_body, &self.upper_body) {
            // Normal 상태
            (LowerBodyState::Idle, UpperBodyState::Normal) => "idle",
            (LowerBodyState::Walk, UpperBodyState::Normal) => "walk",

            // Attack 상태
            (LowerBodyState::Idle, UpperBodyState::Attack) => "attack_fire",
            // TODO: 아직은 attack run 밖에 sprite가 없음 ㅠ
            (LowerBodyState::Walk, UpperBodyState::Attack) => "attack_run",
        }
    }
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
