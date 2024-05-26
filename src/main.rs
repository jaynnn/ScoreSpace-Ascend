use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod bullet;
mod ground;
mod input;
mod player;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            input::input_plugin,
            player::player_plugin,
            bullet::bullet_plugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
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

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(Camera2dBundle::default());
    cmds.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/main_map.ldtk"),
        ..Default::default()
    });
}
