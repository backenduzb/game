use crate::sound::spatial::SPEAKER_POSITION;
use bevy::prelude::*;
use bevy::{
    asset::{AssetServer, Assets},
    color::Color,
    ecs::system::{Commands, ResMut},
    light::DirectionalLight,
    math::{Vec3, primitives::Plane3d},
    mesh::{Mesh, Mesh3d, Meshable},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
    utils::default,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            contact_shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn spawn_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/box_wood.glb")),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.03)),
    ));

    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/speaker_vipe.glb")),
        ),
        Transform::from_xyz(SPEAKER_POSITION.x, 0.0, SPEAKER_POSITION.z)
            .with_scale(Vec3::splat(1.5)),
    ));
}
