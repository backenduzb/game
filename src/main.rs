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
                sound::spatial::spawn_sptial_audio,
            ),
        )
        // .add_systems(Update, camera::controller::update_distance_audio)
        .run();
}
