use avian3d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_just_pressed};
use bevy_editor_cam::DefaultEditorCamPlugins;
use bevy_editor_cam::prelude::{EditorCam, OrbitConstraint};

fn toggle_diagnostics_ui(mut settings: ResMut<PhysicsDiagnosticsUiSettings>) {
    settings.enabled = !settings.enabled;
}

fn physics_paused(time: Res<Time<Physics>>) -> bool {
    time.is_paused()
}

fn toggle_paused(mut time: ResMut<Time<Physics>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

/// Advances the physics simulation by one `Time<Fixed>` time step.
fn step(mut physics_time: ResMut<Time<Physics>>, fixed_time: Res<Time<Fixed>>) {
    physics_time.advance_by(fixed_time.delta());
}

fn setup_key_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("U: Diagnostics UI | P: Pause/Unpause | Enter: Step"),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WheelSide {
    Left,
    Right,
}

impl WheelSide {
    pub fn sign(&self) -> f32 {
        match self {
            WheelSide::Left => 1.0,
            WheelSide::Right => -1.0,
        }
    }
}

#[derive(Resource)]
struct MotorsTorque {
    left_torque: f32,
    right_torque: f32,
}

impl MotorsTorque {
    pub fn new() -> Self {
        Self {
            left_torque: 0.0,
            right_torque: 0.0,
        }
    }

    pub fn torque(&self, side: WheelSide) -> f32 {
        match side {
            WheelSide::Left => self.left_torque,
            WheelSide::Right => self.right_torque,
        }
    }
}

#[derive(Component)]
struct Motors {
    left_axle: Vector,
    right_axle: Vector,
}

#[derive(Component)]
struct Wheel {
    axle: Vector,
    side: WheelSide,
}

fn handle_motors_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut torque: ResMut<MotorsTorque>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let forward = if up {
        1.0
    } else if down {
        -1.0
    } else {
        0.0
    };
    let side = if left {
        -1.0
    } else if right {
        1.0
    } else {
        0.0
    };

    const FORWARD_TORQUE: f32 = 10.1;
    const SIDE_TORQUE: f32 = 10.1;

    torque.left_torque = forward * FORWARD_TORQUE + side * SIDE_TORQUE;
    torque.right_torque = forward * FORWARD_TORQUE - side * SIDE_TORQUE;
}

fn set_wheel_torque(
    torque: Res<MotorsTorque>,
    mut query: Query<(&Wheel, &Transform, &mut ExternalTorque)>,
) {
    for (wheel, transform, mut ext_torque) in &mut query {
        let torque = torque.torque(wheel.side) * wheel.side.sign();
        let wheel_axle = transform.rotation * wheel.axle;
        ext_torque.set_torque(wheel_axle * torque);
    }
}

fn set_motors_torque(
    torque: Res<MotorsTorque>,
    mut query: Query<(&Motors, &Transform, &mut ExternalTorque)>,
) {
    for (motors, transform, mut ext_torque) in &mut query {
        let left_torque = torque.left_torque * WheelSide::Left.sign() * -1.0;
        let left_axle = transform.rotation * motors.left_axle;
        let right_torque = torque.right_torque * WheelSide::Right.sign() * -1.0;
        let right_axle = transform.rotation * motors.right_axle;
        ext_torque.set_torque((left_axle * left_torque) + (right_axle * right_torque));
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            DefaultEditorCamPlugins,
            PhysicsDiagnosticsPlugin,
            PhysicsDiagnosticsUiPlugin,
            PhysicsDebugPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        // Configure the physics debug rendering.
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: None,
                ..PhysicsGizmos::all()
            },
            GizmoConfig::default(),
        )
        // Add gravity to the physics simulation.
        .insert_resource(Gravity(Vec3::NEG_Z * 9.81))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(SubstepCount(50))
        // Resource for motors torque values.
        .insert_resource(MotorsTorque::new())
        // Configure the default physics diagnostics UI.
        .insert_resource(PhysicsDiagnosticsUiSettings {
            enabled: false,
            ..default()
        })
        // Spawn text instructions for keybinds.
        .add_systems(Startup, setup_key_instructions)
        .add_systems(
            RunFixedMainLoop,
            (handle_motors_input, set_wheel_torque, set_motors_torque)
                .chain()
                .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        )
        // Add systems for toggling the diagnostics UI and pausing and stepping the simulation.
        .add_systems(
            Update,
            (
                toggle_diagnostics_ui.run_if(input_just_pressed(KeyCode::KeyU)),
                toggle_paused.run_if(input_just_pressed(KeyCode::KeyP)),
                step.run_if(physics_paused.and(input_just_pressed(KeyCode::Enter))),
            ),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // let wheel_shape: Mesh = Cylinder::default().into();
    // let wheel_mesh = meshes.add(wheel_shape.rotated_by(Quat::from_rotation_z(FRAC_PI_2)));
    let wheel_mesh = meshes.add(Cylinder::default());
    let cube_mesh = meshes.add(Cuboid::default());
    let floor_material = materials.add(Color::srgb(0.4, 0.4, 0.4));
    let body_material = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let wheel_material = materials.add(Color::srgb(0.2, 0.2, 0.2));

    // Static floor
    let _floor = commands
        .spawn((
            Mesh3d(cube_mesh.clone()),
            Collider::cuboid(1.0, 1.0, 1.0),
            MeshMaterial3d(floor_material.clone()),
            RigidBody::Static,
            Friction::new(0.5),
            Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::new(50.0, 50.0, 0.1)),
        ))
        .id();

    // Static car body with motors
    let car_body = commands
        .spawn((
            Mesh3d(cube_mesh.clone()),
            Collider::cuboid(1.0, 1.0, 1.0),
            MeshMaterial3d(body_material.clone()),
            RigidBody::Dynamic,
            Friction::new(0.05).with_combine_rule(CoefficientCombine::Min),
            MassPropertiesBundle::from_shape(&Cuboid::from_size(Vec3::new(3.0, 0.9, 0.5)), 0.5),
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(3.0, 0.9, 0.5)),
            Motors {
                left_axle: Vector::Y,
                right_axle: Vector::NEG_Y,
            },
            ExternalTorque::ZERO,
        ))
        .id();

    // Left wheel
    let left_wheel = commands
        .spawn((
            Mesh3d(wheel_mesh.clone()),
            Collider::cylinder(0.5, 1.0),
            MeshMaterial3d(wheel_material.clone()),
            Transform::from_xyz(0.0, 1.0, 0.0),
            RigidBody::Dynamic,
            Friction::new(0.95).with_combine_rule(CoefficientCombine::Max),
            MassPropertiesBundle::from_shape(&Cuboid::from_length(1.0), 0.5),
            Wheel {
                axle: Vector::Y,
                side: WheelSide::Left,
            },
            ExternalTorque::ZERO,
        ))
        .id();

    // Right wheel
    let right_wheel = commands
        .spawn((
            Mesh3d(wheel_mesh.clone()),
            Collider::cylinder(0.5, 1.0),
            MeshMaterial3d(wheel_material.clone()),
            Transform::from_xyz(0.0, -1.0, 0.0),
            RigidBody::Dynamic,
            Friction::new(0.95).with_combine_rule(CoefficientCombine::Max),
            MassPropertiesBundle::from_shape(&Cuboid::from_length(1.0), 0.5),
            Wheel {
                axle: Vector::NEG_Y,
                side: WheelSide::Right,
            },
            ExternalTorque::ZERO,
        ))
        .id();

    // Connect left wheel
    commands.spawn(
        RevoluteJoint::new(car_body, left_wheel)
            .with_aligned_axis(Vector::Y)
            .with_local_anchor_1(Vector::Y * 1.0 + Vector::X * -0.5),
    );

    // Connect right wheel
    commands.spawn(
        RevoluteJoint::new(car_body, right_wheel)
            .with_aligned_axis(Vector::Y)
            .with_local_anchor_1(Vector::Y * -1.0 + Vector::X * -0.5),
    );

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -1.5, -2.5), Vec3::Z),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        EditorCam {
            orbit_constraint: OrbitConstraint::Fixed {
                up: Vec3::Z,
                can_pass_tdc: false,
            },
            ..Default::default()
        },
        Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
