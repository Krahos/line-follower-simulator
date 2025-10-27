use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

use bevy::ecs::system::Commands;
use bevy::prelude::*;
use execution_data::{BodyExecutionData, WheelExecutionData};
use executor::wasm_bindings::exports::robot::Configuration;

use crate::utils::Side;

use super::motors::Wheel;
use super::{BotBodyMarker, BotConfigurationResource};

pub struct BotMeshes {
    pub body: Handle<Mesh>,
    pub wheel: Handle<Mesh>,
}

pub struct BotMaterials {
    pub body: Handle<StandardMaterial>,
    pub wheel: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct BotAssets {
    pub meshes: BotMeshes,
    pub materials: BotMaterials,
}

pub fn setup_bot_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_mesh = meshes.add(Cuboid::from_size(Vec3::ONE / 2.0));
    let body_material = materials.add(Color::srgb(0.8, 0.2, 0.2));

    let wheel_mesh = meshes.add(Cylinder::new(0.5, 1.0));
    let wheel_material = materials.add(Color::srgb(0.2, 0.8, 0.2));

    let assets = BotAssets {
        meshes: BotMeshes {
            body: body_mesh.clone(),
            wheel: wheel_mesh.clone(),
        },
        materials: BotMaterials {
            body: body_material.clone(),
            wheel: wheel_material.clone(),
        },
    };

    commands.insert_resource(assets);
}

pub fn spawn_bot_body(
    commands: &mut Commands,
    parent: Entity,
    configuration: &Configuration,
    assets: &BotAssets,
    data: Option<BodyExecutionData>,
) -> Entity {
    let id = commands.spawn((ChildOf(parent), Transform::default())).id();
    if let Some(data) = data {
        commands.entity(id).insert(data);
    }

    commands.spawn((
        ChildOf(id),
        Mesh3d(assets.meshes.body.clone()),
        MeshMaterial3d(assets.materials.body.clone()),
        Transform::from_scale(Vec3::new(0.01, 0.01, 0.01)),
    ));
    id
}

pub fn spawn_bot_wheel(
    commands: &mut Commands,
    parent: Entity,
    configuration: &Configuration,
    assets: &BotAssets,
    side: Side,
    data: Option<WheelExecutionData>,
) {
    let transform = data
        .as_ref()
        .map(|data| {
            let t = Transform::from_translation(
                data.side.axis_direction() * configuration.width_axle / 2000.0,
            );
            println!("wheel transform {} {:?}", data.side, t);
            t
        })
        .unwrap_or_default();

    let id = commands.spawn((ChildOf(parent), transform)).id();

    if let Some(data) = data {
        commands.entity(id).insert(data);
    }

    let wheel_d = configuration.wheel_diameter / 1000.0;
    let wheel_w = wheel_d * 3.0 / 2.0;

    // cylinder mesh
    commands.spawn((
        ChildOf(id),
        Mesh3d(assets.meshes.wheel.clone()),
        MeshMaterial3d(assets.materials.wheel.clone()),
        Transform::from_translation(Vec3::X * -side.sign() * (wheel_w - wheel_d) / 2.0)
            .with_scale(Vec3::new(wheel_d, wheel_w, wheel_d))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    ));

    // axle
    let axle_d = 0.003;
    let axle_out = 0.001;
    commands.spawn((
        ChildOf(id),
        Mesh3d(assets.meshes.wheel.clone()),
        MeshMaterial3d(assets.materials.body.clone()),
        Transform::from_scale(Vec3::new(
            axle_d,
            2.0 * wheel_w - wheel_d + 2.0 * axle_out,
            axle_d,
        ))
        .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    ));

    // ext drawing
    let ext_cyl_tranform = Vec3::new(wheel_d / 3.5, axle_out / 2.0, wheel_d / 2.0);
    commands.spawn((
        ChildOf(id),
        Mesh3d(assets.meshes.wheel.clone()),
        MeshMaterial3d(assets.materials.body.clone()),
        Transform::from_translation(Vec3::new(
            -side.sign() * (wheel_w - wheel_d / 2.0),
            wheel_d / 4.0,
            0.0,
        ))
        .with_scale(ext_cyl_tranform)
        .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    ));
    commands.spawn((
        ChildOf(id),
        Mesh3d(assets.meshes.wheel.clone()),
        MeshMaterial3d(assets.materials.body.clone()),
        Transform::from_translation(Vec3::new(
            -side.sign() * (wheel_w - wheel_d / 2.0),
            -(wheel_d / 4.0) * FRAC_PI_3.cos(),
            -(wheel_d / 4.0) * FRAC_PI_3.sin(),
        ))
        .with_scale(ext_cyl_tranform)
        .with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_3, 0.0, FRAC_PI_2)),
    ));
    commands.spawn((
        ChildOf(id),
        Mesh3d(assets.meshes.wheel.clone()),
        MeshMaterial3d(assets.materials.body.clone()),
        Transform::from_translation(Vec3::new(
            -side.sign() * (wheel_w - wheel_d / 2.0),
            -(wheel_d / 4.0) * FRAC_PI_3.cos(),
            (wheel_d / 4.0) * FRAC_PI_3.sin(),
        ))
        .with_scale(ext_cyl_tranform)
        .with_rotation(Quat::from_euler(EulerRot::XYZ, -FRAC_PI_3, 0.0, FRAC_PI_2)),
    ));
}

pub fn setup_test_bot_visualizer(
    mut commands: Commands,
    assets: Res<BotAssets>,
    configuration: Res<BotConfigurationResource>,
    body_query: Query<Entity, With<BotBodyMarker>>,
    wheels_query: Query<(Entity, &Wheel)>,
) {
    let cfg = configuration.cfg();

    let body = body_query.single().unwrap();
    spawn_bot_body(&mut commands, body, &cfg, &assets, None);

    for (wheel_id, wheel) in wheels_query.iter() {
        spawn_bot_wheel(&mut commands, wheel_id, &cfg, &assets, wheel.side, None);
    }
}
