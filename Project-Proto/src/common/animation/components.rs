use bevy::prelude::*;

pub struct AnimationPlugin;

#[derive(Component)]
pub struct AnimationController {
    pub current_frame: usize,
    pub frame_timer: Timer,
    pub direction: Direction8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
