## v0.3.0
### Adjustments
- Updated to `bevy@0.17`

## v0.2.0
### Features
- Added new `GltfSceneHandle::use_animation_transitions` to spawn
  an `AnimationTransition` component alongside your `AnimationPlayer`

### Adjustments
- Updated `idle` example. This is now named `idle_repeat` and calls
  `AnimationPlayer::repeat`.

### Chores
- Updated `bevy_gltf` to respect minor bevy version
- Added `transitions` example
