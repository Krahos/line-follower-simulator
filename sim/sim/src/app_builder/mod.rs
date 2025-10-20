use bevy::prelude::*;
use bevy_editor_cam::DefaultEditorCamPlugins;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::IntegrationParameters;

use crate::bot::add_bot_setup;
use crate::motors::add_motors;
use crate::sensors::add_sensors;
use crate::track::add_track;
use crate::ui::add_ui_setup;

pub fn create_app() -> App {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::default().with_custom_initialization(
            RapierContextInitialization::InitializeDefaultRapierContext {
                rapier_configuration: {
                    let mut config = RapierConfiguration::new(0.001);
                    config.gravity = Vec3::NEG_Z * 9.81;
                    config
                },
                integration_parameters: IntegrationParameters::default(),
            },
        ),
        DefaultEditorCamPlugins,
        RapierDebugRenderPlugin::default(),
    ))
    .insert_resource(Time::<Fixed>::from_hz(10000.0));

    add_track(&mut app);
    add_bot_setup(&mut app);
    add_ui_setup(&mut app);
    add_sensors(&mut app);
    add_motors(&mut app);

    app
}
