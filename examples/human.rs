use bevy::prelude::*;
use bevy_gltf_animation::GltfAnimationPlugin;

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
    commands.spawn((Human, SceneRoot(asset_server.load("human.glb#Scene0"))));
}
