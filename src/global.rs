use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub const RAPIER_LENGTH_UNIT: f32 = 100.0;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GlobalData {
    pub gravity: Vec2,
}

pub fn global_plugin(app: &mut App) {
    app
    .insert_resource(
        GlobalData {
            gravity: Vec2::new(0., -20. * RAPIER_LENGTH_UNIT),
        }
    )
    .register_type::<GlobalData>();
}