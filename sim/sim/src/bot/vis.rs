use bevy::ecs::system::Commands;
use bevy::prelude::*;

use super::BotBodyMarker;
use super::motors::Wheel;

pub fn setup_bot_visualizer(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    body_query: Query<Entity, With<BotBodyMarker>>,
    wheels_query: Query<(Entity, &Wheel)>,
) {
    // Mesh
    let cube_mesh = meshes.add(Cuboid::default());
    let body_material = materials.add(Color::srgb(0.8, 0.2, 0.2));

    let body = body_query.single().unwrap();
    commands.entity(body).insert((
        Mesh3d(cube_mesh.clone()),
        MeshMaterial3d(body_material.clone()),
    ));
}
