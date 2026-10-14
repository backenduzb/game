use bevy::{audio::Volume, prelude::*};

pub const SPEAKER_POSITION: Vec3 = Vec3::new(3.0, 1.2, 0.0);
pub const SPEAKER_VOLUME: Volume = Volume::Linear(100.0);

#[derive(Component)]
pub struct DistanceAudio {
    pub min_distance: f32,
    pub max_distance: f32,
    pub max_volume: f32,
}

pub fn spawn_sptial_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/music.ogg")),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_volume(SPEAKER_VOLUME),
        Transform::from_translation(SPEAKER_POSITION),
    ));
}
