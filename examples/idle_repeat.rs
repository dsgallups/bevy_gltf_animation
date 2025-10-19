use bevy::prelude::*;
use bevy_gltf_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GltfAnimationPlugin))
        .add_systems(Startup, setup)
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
    commands
        .spawn((Human, GltfSceneRoot::new(asset_server.load("human.glb"))))
        .observe(idle);
}

fn idle(
    ev: On<Add, GltfAnimations>,
    mut humans: Query<&mut GltfAnimations>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let Ok(mut gltf_animations) = humans.get_mut(ev.event_target()) else {
        return;
    };
    let mut player = players.get_mut(gltf_animations.animation_player).unwrap();
    let idle_animation = gltf_animations.get_by_name("Idle_Loop").unwrap();
    player.stop_all();
    player.play(idle_animation).repeat();
}
