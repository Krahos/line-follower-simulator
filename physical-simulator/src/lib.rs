use execution_data::{ExecutionData, ExecutionStep};
use rapier3d::{
    na::{Const, OPoint},
    prelude::*,
};

pub const SIMULATION_FREQUENCY_HZ: f32 = 60.0;
pub const SIMULATION_DURATION_S: f32 = 5.0;
pub const SIMULATION_STEPS_COUNT: usize =
    (SIMULATION_FREQUENCY_HZ * SIMULATION_DURATION_S) as usize;

pub const GROUND_SIZE: f32 = 10.0;

pub const BODY_SIZE_X: f32 = 0.2;
pub const BODY_SIZE_Y: f32 = 0.05;
pub const BODY_SIZE_Z: f32 = 0.3;

pub const WHEEL_D: f32 = 0.08;
pub const WHEEL_R: f32 = WHEEL_D / 2.0;
pub const WHEEL_W: f32 = 0.02;

pub const WHEEL_TO_BODY: f32 = 0.01;

pub const WHEEL_TO_BODY_POSITION: f32 = (BODY_SIZE_X + WHEEL_W) / 2.0 + WHEEL_TO_BODY;

pub const START_H: f32 = 0.3;

pub const BODY_POSITION: OPoint<f32, Const<3>> = point![0.0, WHEEL_R + START_H, 0.0];
pub const WHEEL_POSITIONS: [OPoint<f32, Const<3>>; 2] = [
    point![-WHEEL_TO_BODY_POSITION, WHEEL_R + START_H, 0.0],
    point![WHEEL_TO_BODY_POSITION, WHEEL_R + START_H, 0.0],
];

pub fn simulate() -> ExecutionData {
    let mut data = ExecutionData {
        steps: Vec::with_capacity(SIMULATION_STEPS_COUNT),
    };

    /* Create structures necessary for the simulation. */
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let gravity = vector![0.0, -9.81, 0.0];
    let mut integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    // Set simulation dt
    integration_parameters.dt = 1.0 / SIMULATION_FREQUENCY_HZ;

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(GROUND_SIZE, 0.1, GROUND_SIZE).build();
    collider_set.insert(collider);

    // Body
    let body_co = ColliderBuilder::cuboid(BODY_SIZE_X, BODY_SIZE_Y, BODY_SIZE_Z).density(100.0);
    let body_rb = RigidBodyBuilder::dynamic()
        .position(BODY_POSITION.into())
        .build();
    let body_handle = rigid_body_set.insert(body_rb);
    collider_set.insert_with_parent(body_co, body_handle, &mut rigid_body_set);

    let mut wheel_handles = vec![];
    let mut motor_joints = vec![];

    // Wheels
    for wheel_position in WHEEL_POSITIONS {
        let wheel_co = ColliderBuilder::cylinder(WHEEL_W / 2.0, WHEEL_R)
            .rotation(Vector::z() * std::f32::consts::FRAC_PI_2)
            .density(100.0)
            .friction(1.0);
        let wheel_rb = RigidBodyBuilder::dynamic().position(wheel_position.into());
        let wheel_handle = rigid_body_set.insert(wheel_rb);
        collider_set.insert_with_parent(wheel_co, wheel_handle, &mut rigid_body_set);

        // Joint between the body and the wheel
        let wheel_joint = RevoluteJointBuilder::new(Vector::x_axis());
        let wheel_joint_handle =
            impulse_joint_set.insert(body_handle, wheel_handle, wheel_joint, true);

        wheel_handles.push(wheel_handle);
        motor_joints.push(wheel_joint_handle);
    }

    /* Run the game loop, stepping the simulation once per frame. */
    for i in 0..SIMULATION_STEPS_COUNT {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        let body = &rigid_body_set[body_handle];
        let wheel_l = &rigid_body_set[wheel_handles[0]];
        let wheel_r = &rigid_body_set[wheel_handles[1]];

        data.steps.push(ExecutionStep {
            time_s: i as f32 / SIMULATION_FREQUENCY_HZ,
            body_rotation: *body.rotation(),
            body_translation: *body.translation(),
            left_wheel_rotation: *wheel_l.rotation(),
            left_wheel_translation: *wheel_l.translation(),
            right_wheel_rotation: *wheel_r.rotation(),
            right_wheel_translation: *wheel_r.translation(),
        });
    }

    data
}
