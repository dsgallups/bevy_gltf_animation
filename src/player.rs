use std::ffi::OsStr;

use bevy::{asset::LoadedAsset, prelude::*, scene::SceneInstanceReady};

use crate::DoNotInsertGltfAnimationPlayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_gltf_animation_player)
        .add_systems(Update, add_animations);
    //todo
}

#[derive(Component)]
pub struct GltfAnimationPlayer;

#[derive(Component)]
struct LoadingGltf(Handle<Gltf>);

fn add_gltf_animation_player(
    trigger: Trigger<SceneInstanceReady>,
    assets: Res<AssetServer>,
    q_scene_root: Query<&SceneRoot, Without<DoNotInsertGltfAnimationPlayer>>,
    scenes: Res<Assets<Scene>>,
    gltfs: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    let scene_entity = trigger.target();
    let Ok(scene_root) = q_scene_root.get(scene_entity) else {
        return;
    };

    let Some(scene_path) = scene_root.0.path() else {
        return;
    };

    let Some(extension) = scene_path.path().extension().and_then(OsStr::to_str) else {
        return;
    };

    const GLTF_EXTENSIONS: [&str; 2] = ["glb", "gltf"];
    if !GLTF_EXTENSIONS.contains(&extension) {
        return;
    }

    info!("fixing");

    let scene = scenes.get(&scene_root.0).unwrap();
    info!("have scene!");
    // let id = scene_root.0.id().untyped();
    // let Ok(id) = id.try_typed() else {
    //     error!("not gltf");
    //     return;
    // };
    // let gltf = gltfs.get(id).unwrap();

    //let label = path.label()

    info!("Path: {:?}", scene_path.path());

    let animation: Handle<Gltf> = assets.load(scene_path.path());

    commands.entity(scene_entity).insert(LoadingGltf(animation));

    // Only rotate the immediate children of the scene root. Those correspond to the glTF nodes.
    // let mut iter = q_transform.iter_many_mut(children);
    // while let Some(mut transform) = iter.fetch_next() {
    //     transform.rotate_y(PI);
    // }
}

fn add_animations(
    mut commands: Commands,
    gltf_handlers: Query<(Entity, &LoadingGltf)>,
    gltfs: Res<Assets<Gltf>>,
) {
    for (entity, handle) in gltf_handlers {
        let Some(gltf) = gltfs.get(&handle.0) else {
            info!("can't get gltf yet");
            continue;
        };
        info!("Got gltf!\n{gltf:#?}");

        commands.entity(entity).remove::<LoadingGltf>();
        //todo
    }
}
