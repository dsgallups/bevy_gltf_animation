# Easy animations for GLTF files

[<img alt="github" src="https://img.shields.io/badge/github-dsgallups/bevy_gltf_animation?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dsgallups/bevy_gltf_animation)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_gltf_animation.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bevy_gltf_animation)

## Purpose
GLTF files include animations in their files. Importing them into bevy can be tedious, especially if you don't plan to blend any animations.

This crate automatically creates a handler for your animations per scene root based on the imported GLTF file.

## Example

```rust, ignore

use std::time::Duration;

use bevy::prelude::*;
use bevy_gltf_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GltfAnimationPlugin))
        .init_resource::<AnimationTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, play_animations)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawns a SceneRoot, and GltfAnimations on this component
    commands.spawn(GltfSceneRoot::new(asset_server.load("human.glb")));
}
// once gltf animations have been added, play animation 2.
fn play_animations(
    animations: Query<&mut GltfAnimations, Added<GltfAnimations>>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for mut gltf_animations in animations {
        let index = gltf_animations.get(2).unwrap();
        // if named, you can use
        // let index = gltf_animations.get("Walking_Forward").unwrap()
        let mut player = players.get_mut(gltf_animations.animation_player).unwrap();
        player.play(index);
    }
}
```
