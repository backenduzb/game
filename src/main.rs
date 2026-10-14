#![allow(warnings)]

use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
};


mod camera;
mod rendering;
mod sound;
mod ui;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AudioPlugin {
            default_spatial_scale: SpatialScale::new(0.5),
            ..default()
        }))
        .add_plugins(camera::CameraPlugin)
        .add_systems(
            Startup,
            (
                world::scene::setup,
                ui::hud::setup,
                rendering::scene::setup,
                world::scene::spawn_models,
            ),
        )
        .add_systems(
            Update,
            (
                world::scene::debug_gltf_animations,
                world::scene::play_door_animation,
            ),
        )
        .run();
}
