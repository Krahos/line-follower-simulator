use physical_simulator::{
    BODY_SIZE_X, BODY_SIZE_Y, BODY_SIZE_Z, GROUND_SIZE, WHEEL_R, WHEEL_W, simulate,
};
use std::{
    f32::consts::FRAC_PI_2,
    time::{Duration, Instant},
};

use rapier3d::math::{Rotation as RapierQuaternion, Vector as RapierVector3};
use three_d::*;

const RENDERING_FPS: u32 = 60;

pub trait RapierVector3Ext {
    fn into_vec(&self) -> Vec3;
}

impl RapierVector3Ext for RapierVector3<f32> {
    fn into_vec(&self) -> Vec3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

fn cube_base_transformation(scale: Vec3) -> Mat4 {
    Mat4::from_nonuniform_scale(scale.x, scale.y, scale.z)
}

fn cylinder_base_transformation(height: f32, radius: f32) -> Mat4 {
    let scale_t = Mat4::from_nonuniform_scale(height, radius, radius);
    let translate_t = Mat4::from_translation(Vec3 {
        x: -height / 2.0,
        y: 0.0,
        z: 0.0,
    });
    let rotation_t = Mat4::from_angle_z(Rad(FRAC_PI_2));

    scale_t * rotation_t * translate_t
}

fn compute_transformation(
    base: Mat4,
    translate: RapierVector3<f32>,
    rotation: RapierQuaternion<f32>,
) -> Mat4 {
    let translate_t = Mat4::from_translation(translate.into_vec());
    let rotation_t = rotation
        .axis_angle()
        .map(|(axis, angle)| Mat4::from_axis_angle(axis.into_inner().into_vec(), Rad(angle)))
        .unwrap_or_else(|| Mat4::identity());

    base * rotation_t * translate_t
}

fn main() {
    let data = simulate();

    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: None,
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let mut ground = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 100,
                    g: 100,
                    b: 100,
                    a: 50,
                },
                ..Default::default()
            },
        ),
    );
    let mut body = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 0,
                    b: 255,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    let mut wheel_l = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    let mut wheel_r = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 255,
                    b: 255,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    ground.set_transformation(cube_base_transformation(Vec3 {
        x: GROUND_SIZE,
        y: 0.1,
        z: GROUND_SIZE,
    }));
    let body_base_t = cube_base_transformation(Vec3 {
        x: BODY_SIZE_X,
        y: BODY_SIZE_Y,
        z: BODY_SIZE_Z,
    });
    let wheel_base_t = cylinder_base_transformation(WHEEL_W, WHEEL_R);

    let axes = Axes::new(&context, 0.1, 2.0);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, 0.5, 0.5));

    let mut step_index = 0;

    window.render_loop(move |mut frame_input| {
        let step_time = Instant::now();
        let current_step = &data.steps[step_index];

        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        body.set_transformation(compute_transformation(
            body_base_t,
            current_step.body_translation,
            current_step.body_rotation,
        ));
        wheel_l.set_transformation(compute_transformation(
            wheel_base_t,
            current_step.left_wheel_translation,
            current_step.left_wheel_rotation,
        ));
        wheel_r.set_transformation(compute_transformation(
            wheel_base_t,
            current_step.right_wheel_translation,
            current_step.right_wheel_rotation,
        ));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                axes.into_iter()
                    .chain(&body)
                    .chain(&wheel_l)
                    .chain(&wheel_r)
                    .chain(&ground),
                &[&light0, &light1],
            );

        while step_time.elapsed() < Duration::from_micros(1000000 / RENDERING_FPS as u64) {}
        step_index += 1;
        step_index %= data.steps.len();

        FrameOutput::default()
    });
}
