use bevy::prelude::*;

pub mod model;
pub mod motors;
pub mod sensors;
pub mod vis;

use model::setup_bot_model;

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
        app.add_systems(Startup, setup_bot_model);
    }
}

pub fn add_bot_setup(app: &mut App) {
    app.add_plugins(BotPlugin::new(BotFeatures::Physics));
}
