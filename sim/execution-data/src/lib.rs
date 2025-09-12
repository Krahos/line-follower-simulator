use rapier3d::math;

pub struct ExecutionStep {
    pub time_s: f32,
    pub body_rotation: math::Rotation<f32>,
    pub body_translation: math::Vector<f32>,
    pub left_wheel_rotation: math::Rotation<f32>,
    pub left_wheel_translation: math::Vector<f32>,
    pub right_wheel_rotation: math::Rotation<f32>,
    pub right_wheel_translation: math::Vector<f32>,
}

pub struct ExecutionData {
    pub steps: Vec<ExecutionStep>,
}
