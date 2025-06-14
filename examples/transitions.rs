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

#[derive(Component)]
struct Human;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 3.0, 3.0).looking_at(Vec3::Y * 0.5, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::IDENTITY.looking_to(Vec3::new(-0.5, -0.5, -1.0), Vec3::Y),
    ));

    //human
    commands.spawn((
        Human,
        GltfSceneRoot::new(asset_server.load("human.glb")).use_animation_transitions(),
    ));
}

#[derive(Resource)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1), TimerMode::Repeating))
    }
}

fn play_animations(
    mut anim_timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    humans: Query<&mut GltfAnimations, With<Human>>,
    mut index: Local<usize>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    anim_timer.0.tick(time.delta());
    if !anim_timer.0.just_finished() {
        return;
    }

    for mut gltf_animations in humans {
        let (mut player, mut transitions) =
            players.get_mut(gltf_animations.animation_player).unwrap();
        let Some(index) = gltf_animations.get_by_number(*index) else {
            *index = 0;
            return;
        };
        transitions.play(&mut player, index, Duration::from_millis(250));
    }
    *index += 1;
}
