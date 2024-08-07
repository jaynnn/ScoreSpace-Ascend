use bevy::{asset, prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::prelude::*;

mod wall;
mod global;
mod input;
mod player;
mod scene;
mod ldtk;
mod enemy;
mod roulette;
mod menu;
mod animate;
mod bullet;
mod comm;
mod config;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ASCEND".to_string(),
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(LdtkPlugin)
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    })
    .insert_resource(LevelSelection::Uid(0))
    .add_plugins((
        // input::input_plugin,
        player::player_plugin,
        wall::wall_plugin,
        global::global_plugin,
        scene::scene_plugin,
        ldtk::ldtk_plugin,
        enemy::enemy_plugin,
        roulette::roulette_plugin,
        animate::animate_plugin,
        bullet::bullet_plugin,
        config::config_plugin,

        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(global::RAPIER_LENGTH_UNIT),
    ))
    .add_systems(Startup, main_setup);

    app
        .init_state::<AppState>();

    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            // LogDiagnosticsPlugin::default(),
            RapierDebugRenderPlugin::default(),
            EditorPlugin::default()
        ));
    }

    app.run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    Menu,
    #[default]
    GameMain,
    GameSub,
    GameOver,
}

pub fn main_setup(
    mut cmds: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    global_data: ResMut<global::GlobalData>,
    asset_server: Res<AssetServer>,
) {
    cmds.spawn(Camera2dBundle::default()).insert(IsDefaultUiCamera);
    rapier_config.gravity = global_data.gravity;


    // cmds.spawn(LdtkWorldBundle {
    //     ldtk_handle: asset_server.load("ldtk/Typical_2D_platformer_example.ldtk"),
    //     ..Default::default()
    // });
}
