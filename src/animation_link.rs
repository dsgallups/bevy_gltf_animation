use std::iter;

use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::prelude::GltfSceneRoot;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AnimationPlayerOf>();
    app.register_type::<AnimationPlayers>();
    app.add_observer(link_animation_player);
}
