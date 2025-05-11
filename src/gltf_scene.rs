use std::{collections::HashSet, iter};

use bevy::{platform::collections::HashMap, prelude::*, scene::SceneInstanceReady};

pub fn plugin(app: &mut App) {
    app.register_type::<AnimationPlayerOf>();
    app.register_type::<AnimationPlayers>();
    app.add_systems(PreUpdate, load_scene);
}

#[derive(Component)]
pub struct GltfSceneRoot {
    handle: Handle<Gltf>,
    /// Which scene to display
    use_scene: usize,
}

impl GltfSceneRoot {
    pub fn new(handle: Handle<Gltf>) -> Self {
        Self {
            handle,
            use_scene: 0,
        }
    }
    pub fn with_scene(mut self, scene_number: usize) -> Self {
        self.use_scene = scene_number;
        self
    }
}
#[derive(Component)]
pub struct IsSetup;

#[derive(Component)]
pub struct GltfAnimations {
    numbers: HashMap<usize, AnimationNodeIndex>,
    names: HashMap<String, AnimationNodeIndex>,
}

impl GltfAnimations {
    fn new(gltf: &Gltf) -> (Self, AnimationGraph) {
        let mut map = HashMap::new();

        //we're going to reverse this
        for (name, animation) in &gltf.named_animations {
            map.insert(animation.clone(), name.to_string());
        }

        let mut unique_handles = Vec::new();
        for clip in &gltf.animations {
            // remove all names
            let name = map.remove(clip);

            let Some(ext) = clip.path().and_then(|p| p.label()) else {
                error!("No path or label for clip {:?}", clip.id());
                continue;
            };
            let Some(animation_no) = ext
                .strip_prefix("Animation")
                .and_then(|index| index.parse::<usize>().ok())
            else {
                error!("Couldn't parse the animation number for the {ext}");
                continue;
            };

            unique_handles.push((clip.clone(), name, animation_no));
        }

        //idk if this is true
        debug_assert!(map.is_empty());
        let mut animation_graph = AnimationGraph::new();

        let mut number_map = HashMap::new();
        let mut named_map = HashMap::new();

        for (handle, name, number) in unique_handles {
            let node_index = animation_graph.add_clip(handle, 1.0, animation_graph.root);

            number_map.insert(number, node_index);
            if let Some(name) = name {
                named_map.insert(name, node_index);
            }
        }

        let animations = Self {
            numbers: number_map,
            names: named_map,
        };

        (animations, animation_graph)
    }
}

fn load_scene(
    mut commands: Commands,
    gltf_scenes: Query<(Entity, &GltfSceneRoot), Without<IsSetup>>,
    gltfs: Res<Assets<Gltf>>,
) {
    for (entity, scene) in gltf_scenes {
        let Some(gltf) = gltfs.get(&scene.handle) else {
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
        info!("added scene root");
        commands
            .entity(entity)
            .insert((
                SceneRoot(gltf_scene_handle.clone()),
                AnimationPlayerAncestor,
                IsSetup,
            ))
            .observe(setup_animations);
    }
}

/// Entities with this component will receive an [`AnimationPlayers`] relationship so that they can easily find the animation player of their model.
#[derive(Component)]
pub(crate) struct AnimationPlayerAncestor;

/// Simple link to the animation player of a model that is buried deep in the hierarchy.
#[derive(Component, Reflect, Clone, Deref)]
#[reflect(Component)]
#[relationship_target(relationship = AnimationPlayerOf)]
pub(crate) struct AnimationPlayers(Vec<Entity>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship(relationship_target = AnimationPlayers)]
pub(crate) struct AnimationPlayerOf(pub(crate) Entity);

// notes: this doesn't trigger if you don't insert scene root. WHEW.
/// Bevy likes to hide the [`AnimationPlayer`] component deep in the hierarchy of a model.
/// This system ensures that we can find the animation player easily by inserting an [`AnimationPlayers`] relationship
/// into the same entity that contains the [`AnimationPlayerAncestor`] component.
fn setup_animations(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    animation_players: Query<Entity, With<AnimationPlayer>>,
    scene_root: Query<(Entity, &GltfSceneRoot)>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("lil observer");
    let target = trigger.target();
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
        info!("No ancestor");
        return;
    };

    let Some(gltf) = gltfs.get(&gltf_scene_root.handle) else {
        error!("Couldn't find GLTF for Scene root!");
        return;
    };

    let animations = GltfAnimations::new(gltf);

    let (graph, indices) = AnimationGraph::from_clips(gltf.animations.clone());

    let graph_handle = graphs.add(graph);

    commands
        .entity(animation_player)
        .insert(AnimationGraphHandle(graph_handle));

    // commands
    //     .entity(animation_player)
    //     .insert(AnimationPlayerOf(animation_ancestor));
}
