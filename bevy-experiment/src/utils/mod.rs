use bevy::math::{EulerRot, Vec2, Vec3};
use bevy::transform::components::GlobalTransform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn sign(&self) -> f32 {
        match self {
            Side::Left => 1.0,
            Side::Right => -1.0,
        }
    }
}

/// Helper to rotate a Vec2 by angle in radians
/// # Arguments
/// * `v`     - The vector to rotate
/// * `angle` - The angle in radians
pub fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    let (s, c) = angle.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

pub fn point_to_new_origin(point: Vec3, transform: &GlobalTransform) -> Vec2 {
    rotate_vec2(
        (point - transform.translation()).truncate(),
        -transform.rotation().to_euler(EulerRot::ZYX).0,
    )
}
