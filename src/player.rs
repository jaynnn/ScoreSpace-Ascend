use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::input;

#[derive(Component)]
pub struct Player;

fn player_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_player,
    ));
}

fn spawn_player(
    mut cmds: Commands,

) {
    let input_map = InputMap::new(input::PlayerInputMap::default());
    
    let sprite_size = Vec2::splat(16.);
    cmds.spawn((
        Name::new("Player"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(sprite_size),
                ..default()
            },
            ..default()
        },
        Player,
        InputManagerBundle::with_map(input_map),
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
    ));
}