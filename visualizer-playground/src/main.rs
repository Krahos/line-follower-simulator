use physical_simulator::simulate;
use std::time::{Duration, Instant};

use three_d::*;

const RENDERING_FPS: u32 = 60;

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

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 0.0)) * Mat4::from_scale(0.2));

    let axes = Axes::new(&context, 0.1, 2.0);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, 0.5, 0.5));

    let start_time = Instant::now();

    window.render_loop(move |mut frame_input| {
        let step_time = Instant::now();

        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let mut y = 0.0;
        for step in data.steps.iter() {
            if (step_time - start_time).as_secs_f32() >= step.time_s {
                y = step.y;
            }
        }

        sphere
            .set_transformation(Mat4::from_translation(vec3(0.0, y, 0.0)) * Mat4::from_scale(0.2));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                sphere.into_iter().chain(&axes),
                &[&light0, &light1],
            );

        while step_time.elapsed() < Duration::from_micros(1000000 / RENDERING_FPS as u64) {}

        FrameOutput::default()
    });
}
