use bevy::prelude::*;

fn wall_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_wall,
    ));
}

fn spawn_wall(
    mut cmds: Commands,
) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(10., 1000.)),
                ..default()
            },
            transform: Transform::from_xyz(-100., 0., 0.),
            ..default()
        },
        RigidBody::Fixed,
    ));
}