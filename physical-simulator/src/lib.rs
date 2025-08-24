use execution_data::{ExecutionData, ExecutionStep};
use rapier3d::prelude::*;

const SIMULATION_FREQUENCY_HZ: f32 = 60.0;
const SIMULATION_DURATION_S: f32 = 5.0;
const SIMULATION_STEPS_COUNT: usize = (SIMULATION_FREQUENCY_HZ * SIMULATION_DURATION_S) as usize;

pub fn simulate() -> ExecutionData {
    let mut data = ExecutionData {
        steps: Vec::with_capacity(SIMULATION_STEPS_COUNT),
    };

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
    collider_set.insert(collider);

    /* Create the bounding ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0, 0.0])
        .build();
    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
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

    integration_parameters.dt = 1.0 / SIMULATION_FREQUENCY_HZ;

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

        let ball_body = &rigid_body_set[ball_body_handle];

        data.steps.push(ExecutionStep {
            time_s: i as f32 / SIMULATION_FREQUENCY_HZ,
            x: 0.0,
            y: ball_body.translation().y,
            z: 0.0,
        });
    }

    data
}
