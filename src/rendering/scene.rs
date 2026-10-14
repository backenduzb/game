use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::light::DirectionalLight;
use bevy::math::Vec3;
use bevy::math::primitives::Cuboid;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::info;
use bevy::transform::components::Transform;
use bevy::utils::default;
use bevy::{
    asset::Assets,
    ecs::system::ResMut,
    mesh::{Mesh, Mesh3d},
    pbr::StandardMaterial,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("3D scinence loading");

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.9, 0.25, 0.12))),
    //     Transform::from_xyz(0.0, 1.0, 0.0),
    // ));

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.9, 0.25, 0.12))),
    //     Transform::from_xyz(0.0, 1.0, 0.0),
    // ));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            contact_shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
