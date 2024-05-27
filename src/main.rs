use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


mod input;
mod player;
mod bullet;
mod ground;
mod wall;
mod global;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ASCEND".to_string(),
            resolution: WindowResolution::new(1080., 1920.),
            canvas: Some("#bevy".to_owned()),
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        input::input_plugin,
        player::player_plugin,
        bullet::bullet_plugin,
        ground::ground_plugin,
        wall::wall_plugin,
        global::global_plugin,

        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(global::RAPIER_LENGTH_UNIT),
    ))
    .add_systems(Startup, setup);

    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            // LogDiagnosticsPlugin::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    }

    app.run();
}

fn setup(
    mut cmds: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    global_data: ResMut<global::GlobalData>,
) {
    cmds.spawn(Camera2dBundle::default());
    rapier_config.gravity = global_data.gravity;
}
