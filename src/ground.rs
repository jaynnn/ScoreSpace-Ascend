use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn ground_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_ground,
    ));
}

fn spawn_ground(
    mut cmds: Commands,
) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(1000., 10.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -100., 0.),
            ..default()
        },
        RigidBody::Fixed,
    ));
}