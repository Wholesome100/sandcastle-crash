/*
    I think a fun and doable game would be a sand castle game.
    Essentially the player controls a small shovel on a small island, with tools to help them build out a sandcastle.
    At different intervals waves wash over and threaten to destroy their sandcastle.
    Would be simple enough with a static camera, the biggest hurtle being implementing the particle systems.
    I need to physically simulate sand and water.
 */

use bevy::{prelude::*};

mod game_camera;

use crate::game_camera::{CameraPlugin, Ground};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}