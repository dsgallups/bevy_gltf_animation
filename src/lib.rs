use std::iter;

use bevy::{prelude::*, scene::SceneInstanceReady};
use prelude::{GltfAnimations, GltfSceneRoot};

pub mod gltf_scene;

pub struct GltfAnimationPlugin;

#[derive(Component)]
pub struct DoNotInsertGltfAnimationPlayer;

impl Plugin for GltfAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, load_scene);

        #[cfg(feature = "extended")]
        app.add_systems(PostUpdate, play_requested_animations);
    }
}

pub mod prelude {
    pub use crate::gltf_scene::*;
    pub use crate::{DoNotInsertGltfAnimationPlayer, GltfAnimationPlugin};
}

#[derive(Component)]
struct IsSetup;

fn load_scene(
    mut commands: Commands,
    gltf_scenes: Query<
        (Entity, &GltfSceneRoot, Has<DoNotInsertGltfAnimationPlayer>),
        Without<IsSetup>,
    >,
    gltfs: Res<Assets<Gltf>>,
) {
    for (entity, scene, dont_insert_animation_player) in gltf_scenes {
        let Some(gltf) = gltfs.get(&scene.handle) else {
            debug!("gltf not ready!");
            continue;
        };

        let Some(gltf_scene_handle) = gltf.scenes.get(scene.use_scene) else {
            error!(
                "Gltf does not have scene {}! Aborting setup.",
                scene.use_scene
            );
            commands.entity(entity).insert(IsSetup);
            continue;
        };
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert((SceneRoot(gltf_scene_handle.clone()), IsSetup));
        if !dont_insert_animation_player {
            entity_commands.observe(setup_animations);
        }
    }
}

fn setup_animations(
    ev: On<SceneInstanceReady>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    animation_players: Query<Entity, With<AnimationPlayer>>,
    scene_root: Query<(Entity, &GltfSceneRoot)>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let target = ev.event_target();
    let Some(animation_player) = children
        .iter_descendants(target)
        .find(|child| animation_players.get(*child).is_ok())
    else {
        error!("No animation player for scene!");
        return;
    };

    let scene_root = iter::once(animation_player)
        .chain(parents.iter_ancestors(animation_player))
        .find_map(|entity| scene_root.get(entity).ok());
    let Some((scene_root, gltf_scene_root)) = scene_root else {
        error!("No gltf ancestor to attach!");
        return;
    };

    let Some(gltf) = gltfs.get(&gltf_scene_root.handle) else {
        error!("Couldn't find GLTF for Scene root!");
        return;
    };

    let (animations, graph) = GltfAnimations::new(gltf, animation_player);

    let graph_handle = graphs.add(graph);

    let mut animation_player_commands = commands.entity(animation_player);

    animation_player_commands.insert(AnimationGraphHandle(graph_handle));

    if gltf_scene_root.use_animation_transitions {
        animation_player_commands.insert(AnimationTransitions::new());
    }

    commands.entity(scene_root).insert(animations);
}

#[cfg(feature = "extended")]
fn play_requested_animations(
    mut animations: Query<&mut GltfAnimations>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for mut animation in &mut animations {
        let Some(index) = animation.animation_to_play.take() else {
            continue;
        };

        let mut player = players.get_mut(animation.animation_player).unwrap();
        player.stop_all();
        player.play(index);
    }
}
