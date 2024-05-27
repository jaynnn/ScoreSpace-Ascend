use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn ground_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_ground,
    ));
}

fn spawn_ground(
    mut cmds: Commands,
) {
    let sprite_size = Vec2::new(1000., 100.);
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform: Transform::from_xyz(0., -300., 0.),
            ..default()
        },
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
    ));
}