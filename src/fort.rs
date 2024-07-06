use bevy::{ecs::query, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Component)]
pub struct Fort {
    jump_bullet_timer: Timer,
}

impl Fort {
    pub fn new() -> Self {
        Self {
            jump_bullet_timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}


#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct FortData {
    pub jump_bullet_velocity: f32,
}

#[derive(Component)]
pub struct JumpBullet;

pub fn fort_plugin(app: &mut App) {
    app
    .insert_resource(FortData { 
        jump_bullet_velocity: 100.,
    })
    .register_type::<FortData>()
    .add_plugins(ResourceInspectorPlugin::<FortData>::default())
    .add_systems(Startup, (
        spawn_fort,
    ))
    .add_systems(Update, (
        fort_update,
        jump_bullet_update,
    ));
}

fn spawn_fort(
    mut cmds: Commands,
) {
    let sprite_size = Vec2::new(10., 10.);
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform: Transform::from_xyz(-490., -130., 0.),
            ..default()
        },
        Fort::new(),
    ));
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(sprite_size),
                ..default()
            },
            transform: Transform::from_xyz(490., -30., 0.),
            ..default()
        },
        Fort::new(),
    ));
}

fn fort_update(
    mut cmds: Commands,
    mut query_fort: Query<(&mut Fort, &Transform)>,
    time: Res<Time>,
    fort_data: Res<FortData>,
) {
    for (mut fort , transform) in query_fort.iter_mut() {
        fort.jump_bullet_timer.tick(time.delta());
        if fort.jump_bullet_timer.finished() {
            // println!("jump_bullet_timer finished");
            let mut vel = Vec2::new(-fort_data.jump_bullet_velocity, 0.);
            if transform.translation.x < 0. {
                vel = Vec2::new(fort_data.jump_bullet_velocity, 0.);
            }
            cmds.spawn((
                Name::new("JumpBullet"),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(5., 5.)),
                        ..default()
                    },
                    transform : *transform,
                    ..default()
                },
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(10., 10.),
                JumpBullet,
                Velocity::linear(vel),
                Sensor,
            ));
        }
    }
}

fn jump_bullet_update(
    mut cmds: Commands,
    mut query_jump_bullet: Query<(Entity, &mut Transform, &Velocity, &JumpBullet)>,
) {
    // for (entity, mut transform, velocity, _) in query_jump_bullet.iter_mut() {
    //     transform.translation += velocity.linear * 0.02;
    //     if transform.translation.y > 120. {
    //         cmds.entity(entity).despawn();
    //     }
    // }
}

fn jump_bullet_collie_wall(

) {

}