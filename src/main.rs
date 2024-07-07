use bevy::{asset, prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::prelude::*;

mod animate;
mod bullet;
mod comm;
mod config;
mod enemy;
mod global;
mod input;
mod ldtk;
mod menu;
mod player;
mod roulette;
mod scene;
mod wall;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "colorize ".to_string(),
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
        .insert_resource(LevelSelection::Uid(88))
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

    app.init_state::<AppState>();

    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            // LogDiagnosticsPlugin::default(),
            RapierDebugRenderPlugin::default(),
            EditorPlugin::default(),
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
    cmds.spawn(Camera2dBundle::default())
        .insert(IsDefaultUiCamera);
    rapier_config.gravity = global_data.gravity;

    cmds.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ..default()
    },))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style {
                    justify_content: JustifyContent::FlexEnd,
                    top: Val::Px(30.0),
                    right: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                    },
                    ..default()
                },
                text: Text::from_section(
                    "wasd to move, space to jump, R to restart, LMB to shot ice \n wasd移动, 空格跳跃, R重开此关, 鼠标左键发射冰块",
                    TextStyle {
                        font: asset_server.load("fonts/han_rounded.ttf"),
                        font_size: 35.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            },));
        });
    cmds.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/level.ldtk"),
        ..Default::default()
    });
}
