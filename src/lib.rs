use bevy::prelude::*;

pub struct GltfAnimationPlugin;

impl Plugin for GltfAnimationPlugin {
    fn build(&self, app: &mut App) {
        //todo
    }
}

pub mod prelude {
    pub use crate::GltfAnimationPlugin;
}
