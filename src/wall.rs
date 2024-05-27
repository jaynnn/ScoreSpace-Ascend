use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn wall_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_wall,
    ));
}

fn spawn_wall(
    mut cmds: Commands,
) {
    let sprite_size = Vec2::new(10., 1000.);
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform: Transform::from_xyz(-500., 0., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
    ));

    
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(sprite_size),
                
                ..default()
            },
            transform: Transform::from_xyz(500., 0., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
    ));
}