use bevy::audio::{SpatialAudioSink, Volume};
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};
use crate::sound::spatial::DistanceAudio;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FreeCameraPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        SpatialListener::new(4.0),
        FreeCamera {
            walk_speed: 6.0,
            run_speed: 15.0,
            sensitivity: 0.2,
            ..default()
        },
    ));
}

pub fn update_distance_audio(
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
    mut audio_query: Query<(&GlobalTransform, &DistanceAudio, &mut SpatialAudioSink)>,
) {
    let Ok(listener) = listener_query.single() else {
        return;
    };

    let listener_position = listener.translation();

    for (source_transform, audio, mut sink) in &mut audio_query {
        let distance = listener_position.distance(source_transform.translation());

        let volume = if distance <= audio.min_distance {
            audio.max_volume
        } else if distance >= audio.max_distance {
            0.0
        } else {
            let factor =
                1.0 - (distance - audio.min_distance) / (audio.max_distance - audio.min_distance);

            audio.max_volume * factor
        };

        sink.set_volume(Volume::Linear(volume));
    }
}
