use bevy::prelude::*;

pub mod model;
pub mod motors;
pub mod sensors;
pub mod vis;

use model::setup_bot_model;
use motors::Wheel;
use vis::setup_bot_visualizer;

use crate::utils::Side;

pub enum BotFeatures {
    Physics,
    Visualization,
    PhysicsAndVisualization,
}

impl BotFeatures {
    pub fn has_physics(&self) -> bool {
        match self {
            BotFeatures::Physics => true,
            BotFeatures::Visualization => false,
            BotFeatures::PhysicsAndVisualization => true,
        }
    }

    pub fn has_visualization(&self) -> bool {
        match self {
            BotFeatures::Physics => false,
            BotFeatures::Visualization => true,
            BotFeatures::PhysicsAndVisualization => true,
        }
    }
}

pub struct BotPlugin {
    features: BotFeatures,
}

impl BotPlugin {
    pub fn new(features: BotFeatures) -> Self {
        Self { features }
    }
}

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bot_entities);
        if self.features.has_physics() {
            app.add_systems(Startup, setup_bot_model.after(setup_bot_entities));
        }
        if self.features.has_visualization() {
            app.add_systems(Startup, setup_bot_visualizer.after(setup_bot_entities));
        }
    }
}

#[derive(Component)]
pub struct BotBodyMarker;

pub fn setup_bot_entities(mut commands: Commands) {
    commands.spawn(BotBodyMarker);
    for side in [Side::Left, Side::Right] {
        commands.spawn(Wheel::new(Vec3::NEG_X * side.sign(), side));
        commands.spawn(Wheel::new(Vec3::NEG_X * side.sign(), side));
    }
}

pub fn add_bot_setup(app: &mut App) {
    app.add_plugins(BotPlugin::new(BotFeatures::PhysicsAndVisualization));
}
