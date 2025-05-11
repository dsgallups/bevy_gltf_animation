use bevy::prelude::*;

pub mod player;

pub mod gltf_scene;

pub struct GltfAnimationPlugin;

#[derive(Component)]
pub struct DoNotInsertGltfAnimationPlayer;

impl Plugin for GltfAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::plugin, gltf_scene::plugin));
    }
}

pub mod prelude {
    pub use crate::gltf_scene::*;
    pub use crate::player::*;
    pub use crate::{DoNotInsertGltfAnimationPlayer, GltfAnimationPlugin};
}
