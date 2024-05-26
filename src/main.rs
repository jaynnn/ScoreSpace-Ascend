use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


mod input;
mod player;
mod bullet;
mod ground;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins)
    .add_plugins((
        input::input_plugin,
        player::player_plugin,
        bullet::bullet_plugin,

        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
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
    mut cmds: Commands
) {
    cmds.spawn(Camera2dBundle::default());
}
