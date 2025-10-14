use bevy::utils::HashMap;

use crate::common::animation::components::{AnimationClip, AnimationSet};

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
