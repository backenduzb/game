use bevy::prelude::info;
use bevy::{
    color::Color,
    ecs::{children, system::Commands},
    text::{TextColor, TextFont},
    ui::{Node, px, widget::Text},
    utils::default,
};

pub fn setup(mut commands: Commands) {
    info!("UI SETUP CALLED");
    commands.spawn((
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            width: bevy::ui::percent(100),
            height: bevy::ui::percent(100),
            ..default()
        },
        children![(
            Text::new("START GAME"),
            TextFont {
                font_size: bevy::text::FontSize::Px(32.0),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    ));

    commands.spawn((
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            bottom: px(20),
            left: px(20),
            ..default()
        },
        children![(
            Text::new("3D SCIENCE"),
            TextFont {
                font_size: bevy::text::FontSize::Px(12.0),
                ..default()
            },
            TextColor(Color::srgb(0.65, 0.68, 0.72)),
        )],
    ));

    info!("UI SETUP FINISHED");
}
