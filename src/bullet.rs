use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::roulette::{self, Roulette, RouletteItem};

pub fn bullet_plugin(app: &mut App) {
    app.add_event::<BulletEvent>()
        .add_systems(Update, (run, bullet_linstener));
}

#[derive(Component)]
pub struct AtkNormal;

pub fn run(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, With<AtkNormal>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _flags)
            | CollisionEvent::Stopped(entity1, entity2, _flags) => {
                let bullet_entity = if query.contains(*entity1) {
                    *entity1
                } else if query.contains(*entity2) {
                    *entity2
                } else {
                    continue;
                };
                let _other_entity = if bullet_entity == *entity1 {
                    *entity2
                } else {
                    *entity1
                };
            }
        }
    }
}

#[derive(Event)]
pub struct BulletEvent {
    pub transform: Transform,
    pub vel: Vec2,
}

fn bullet_linstener(
    mut cmds: Commands,
    mut events: EventReader<BulletEvent>,
    roulette: Query<(&Roulette)>,
    roulette_item: Query<(&Transform, &Handle<Image>), With<RouletteItem>>,
    query: Query<Entity, With<AtkNormal>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let sprite_size = Vec2::new(16., 16.);
        let transform = event.transform;
        let vel = event.vel;
        let roulette = roulette.single();
        if let Some(cur_item) = roulette.get_cur_item() {}
        spawn_atk_normal(&mut cmds, &transform, vel, sprite_size, &query, &asset_server);
    }
}

pub fn spawn_atk_normal(
    cmds: &mut Commands,
    transform: &Transform,
    vel: Vec2,
    sprite_size: Vec2,
    query: &Query<Entity, With<AtkNormal>>,
    asset_server: &Res<AssetServer>,
) {
    for ice in query {
        cmds.entity(ice).despawn();
    }
    cmds.spawn((
        Name::new("AtkNormal"),
        SpriteBundle {
            sprite: Sprite { 
                custom_size: Some(sprite_size),
                ..default()
            },
            texture: asset_server.load("images/ice.png"),
            transform: transform.clone(),
            ..default()
        },
        // Sensor,
        AtkNormal,
        RigidBody::Dynamic,
        Velocity::linear(vel),
        Collider::cuboid(sprite_size.x / 2., sprite_size.y / 2.),
        ActiveEvents::COLLISION_EVENTS,
    ));
}
