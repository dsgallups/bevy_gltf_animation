use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Component)]
pub struct GltfSceneRoot {
    pub handle: Handle<Gltf>,
    /// Which scene to display
    pub use_scene: usize,
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

#[derive(Component, Debug)]
pub struct GltfAnimations {
    numbers: HashMap<usize, AnimationNodeIndex>,
    names: HashMap<String, AnimationNodeIndex>,
    // used in a post update system and then cleared
    #[cfg(feature = "extended")]
    pub(crate) animation_to_play: Option<AnimationNodeIndex>,
    pub animation_player: Entity,
}

impl GltfAnimations {
    pub(crate) fn new(gltf: &Gltf, animation_player: Entity) -> (Self, AnimationGraph) {
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
            #[cfg(feature = "extended")]
            animation_to_play: None,
            animation_player,
        };

        (animations, animation_graph)
    }

    /// Get an animation by its gltf ID
    pub fn get_by_number(&mut self, index: usize) -> Option<AnimationNodeIndex> {
        self.numbers.get(&index).copied()
    }

    /// Get an animation node index by its gltf name
    pub fn get_by_name(&mut self, index: &str) -> Option<AnimationNodeIndex> {
        self.names.get(index).copied()
    }

    /// Returns the animation node index for the graph. This is better for animations
    /// that need to be played immediately, or with a transition.
    pub fn get<'a>(
        &mut self,
        index: impl Into<GltfAnimationIndexQuery<'a>>,
    ) -> Option<AnimationNodeIndex> {
        match index.into() {
            GltfAnimationIndexQuery::Name(v) => self.names.get(v).copied(),
            GltfAnimationIndexQuery::Number(n) => self.numbers.get(&n).copied(),
        }
    }

    /// a simple helper that will allow you to play an animation.
    /// doing this will run in `PostUpdate`
    #[cfg(feature = "extended")]
    pub fn play<'a>(&mut self, index: impl Into<GltfAnimationIndexQuery<'a>>) {
        let Some(value) = self.get(index) else {
            return;
        };
        self.animation_to_play = Some(value);
    }
}

pub enum GltfAnimationIndexQuery<'a> {
    Name(&'a str),
    Number(usize),
}

impl<'a> From<&'a str> for GltfAnimationIndexQuery<'a> {
    fn from(value: &'a str) -> Self {
        Self::Name(value)
    }
}

impl From<usize> for GltfAnimationIndexQuery<'_> {
    fn from(value: usize) -> Self {
        Self::Number(value)
    }
}
