use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn bullet_plugin(app: &mut App) {
    // app
    // .add_systems(Startup, (
    //     left_run,
    // ));
}

#[derive(Component)]
pub struct LeftShoot;

pub enum BulletType {
    Left,
    Right,
}

pub fn spawn_bullet(
    cmds: &mut Commands,
    bullet_type: BulletType,
    transform: &Transform,
    vel: Vec2,
    sprite_size: Vec2,
) {
    match bullet_type {
        BulletType::Left => spawn_shoot_left(cmds, transform, vel, sprite_size),
        BulletType::Right => spawn_shoot_right(cmds, transform, vel, sprite_size),
    }
}

pub fn spawn_shoot_left(
    cmds: &mut Commands,
    transform: &Transform,
    vel: Vec2,
    sprite_size: Vec2,
) {
    cmds.spawn((
        Name::new("AtkNormal"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform: transform.clone(),
            ..default()
        },
        Sensor,
        LeftShoot,
        RigidBody::Dynamic,
        Velocity::linear(vel),
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn spawn_shoot_right(
    cmds: &mut Commands,
    transform: &Transform,
    vel: Vec2,
    sprite_size: Vec2,
) {

}


// pub fn left_run(
//     mut collision_events: EventReader<CollisionEvent>,
//     query: Query<Entity, With<LeftShoot>>,
// ) {
//     for event in collision_events.read() {
//         match event {
//             CollisionEvent::Started(entity1, entity2, _flags)
//             | CollisionEvent::Stopped(entity1, entity2, _flags) => {
//                 let bullet_entity = if query.contains(*entity1) {
//                     *entity1
//                 } else if query.contains(*entity2) {
//                     *entity2
//                 } else {
//                     continue;
//                 };
//                 let _other_entity = if bullet_entity == *entity1 {
//                     *entity2
//                 } else {
//                     *entity1
//                 };
//             }
//         }
//     }
// }