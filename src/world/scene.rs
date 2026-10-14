// use crate::sound::spatial::SPEAKER_POSITION;
use bevy::prelude::*;
use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Commands, ResMut},
    light::DirectionalLight,
    math::Vec3,
    mesh::Mesh,
    pbr::StandardMaterial,
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
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/door.glb"))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
    ));
    // debug_gltf_animations dagi AssetEvent<Gltf> ishlashi uchun
    // to'liq Gltf ni ham yuklab, handle ni saqlab qo'yamiz.
    // Aks holda faqat Scene(0) yuklanadi va Gltf event hech qachon kelmaydi.
    let gltf_handle: Handle<Gltf> = asset_server.load("models/door.glb");
    commands.insert_resource(DoorGltfHandle(gltf_handle));
}

#[derive(Resource)]
pub struct DoorGltfHandle(pub Handle<Gltf>);

// pub fn play_door_animation(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
//     mut graphs: ResMut<Assets<AnimationGraph>>,
//     asset_server: Res<AssetServer>,
// ) {
//     for (entity, mut player) in &mut query {
//         let animation_handle =
//             asset_server.load(GltfAssetLabel::Animation(11).from_asset("models/door.glb"));
//         let (graph, animation_node) = AnimationGraph::from_clip(animation_handle);
//         let graph_handle = graphs.add(graph);

//         commands
//             .entity(entity)
//             .insert(AnimationGraphHandle(graph_handle));
//         player.play(animation_node).repeat();
//     }
// }
// 
pub fn play_door_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut player) in &mut query {
        let anim_1 = asset_server.load(GltfAssetLabel::Animation(11).from_asset("models/door.glb"));
        let anim_2 = asset_server.load(GltfAssetLabel::Animation(5).from_asset("models/door.glb"));

        let mut graph = AnimationGraph::new();
        let node_index_1 = graph.add_clip(anim_1, 1.0, graph.root);
        let node_index_2 = graph.add_clip(anim_2, 1.0, graph.root);

        let graph_handle = graphs.add(graph);

        commands.entity(entity).insert(AnimationGraphHandle(graph_handle));

        player.play(node_index_1);
        player.play(node_index_2);
    }
}

pub fn debug_gltf_animations(
    door_handle: Option<Res<DoorGltfHandle>>,
    gltf_assets: Res<Assets<Gltf>>,
    clip_assets: Res<Assets<AnimationClip>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(door_handle) = door_handle else {
        return;
    };
    let Some(gltf) = gltf_assets.get(&door_handle.0) else {
        return;
    };
    println!("=== GLTF ANIMATIONS DEBUG ===");
    println!(
        "named_animations: {:?}",
        gltf.named_animations.keys().collect::<Vec<_>>()
    );
    if gltf.animations.is_empty() {
        println!("Diqqat: Bu .glb fayl ichida hech qanday animatsiya topilmadi!");
    } else {
        for (index, anim_handle) in gltf.animations.iter().enumerate() {
            if let Some(clip) = clip_assets.get(anim_handle) {
                println!(
                    "Animatsiya [{}] -> Topildi, duration={:?}, paths={}",
                    index,
                    clip.duration(),
                    clip.curves().len()
                );
            } else {
                println!("Animatsiya [{}] -> Topildi (clip hali yuklanmagan)", index);
            }
        }
    }
    println!("=============================");
    *done = true;
}