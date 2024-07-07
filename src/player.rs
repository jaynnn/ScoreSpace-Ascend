use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::utils::HashSet;

use crate::bullet::BulletEvent;
use crate::roulette::RouletteRotateEvent;
use crate::bullet::spawn_atk_normal;

use crate::scene::Climbable;
use crate::scene::ColliderBundle;
use crate::scene::GroundSensor;
use crate::scene::Items;
use crate::animate::PlayerAnimateEvent;

pub fn player_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_player,
    ))
    .add_systems(Update, on_spawn_player)
    .add_systems(
        Update,
        (player_move, detect_climb_range, ignore_gravity_if_climbing, cursor_move),
    );
}

#[derive(Component, Clone, Default)]
pub struct Player;

pub fn on_spawn_player(mut commands: Commands, mut players: Query<(Entity), Added<Player>>) {
    for player_entity in players.iter_mut() {
        commands
            .entity(player_entity)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(GroundSensor {
                ground_detection_entity: player_entity,
                intersecting_ground_entities: HashSet::new(),
                on_ground: false,
            });
    }
}
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Items,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

fn cursor_move(
    camera_query: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    gizmos.circle_2d(point, 10., Color::WHITE);
}

fn spawn_player(
    mut cmds: Commands,
) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player,
        ColliderBundle {
            collider: Collider::cuboid(36.75, 130.),
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            ..default()
        },
        Climber::default(),
        Name::new("player"),
    ));
}

fn player_move(
    mut cmds: Commands,
    input: Res<ButtonInput<KeyCode>>,
    input_mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&Transform, &mut Velocity, &mut Climber, &GroundSensor), With<Player>>,
    mut roulette_event: EventWriter<RouletteRotateEvent>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    windows: Query<&Window>,
    mut animate_event: EventWriter<PlayerAnimateEvent>,
    mut bullet_event: EventWriter<BulletEvent>,
) {
    for (transform, mut velocity, mut climber, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

            velocity.linvel.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.;
            climber.climbing = false;
        }
        if velocity.linvel.x != 0. {
            animate_event.send(PlayerAnimateEvent::Walk(velocity.linvel));
        }

        if input_mouse_button.just_pressed(MouseButton::Left) {
            if let Some(cursor_position) = windows.single().cursor_position() {
                for (camera, camera_transform) in camera_query.iter() {
                    if let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                        let direction = get_mouse_direction(transform, point);
                        let vel = direction * 1000.;
                        bullet_event.send(BulletEvent { transform: *transform, vel});
                    }
                }
            }
        }

        for event in mouse_wheel_events.read() {
            match event.y {
                1.0 => {
                    roulette_event.send(RouletteRotateEvent::Left);
                }
                -1.0 => {
                    roulette_event.send(RouletteRotateEvent::Right);
                }
                _ => {}
            }
        }
    }
}

fn get_mouse_direction(transform: &Transform, cursor_position: Vec2) -> Vec2 {
    let cursor_position = Vec2::new(cursor_position.x, cursor_position.y);
    let player_position = transform.translation.xy();
    let direction = cursor_position - player_position;
    direction.normalize()
}

pub fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_a), climbables.get(*collider_b))
                {
                    climber.intersecting_climbables.insert(climbable);
                }
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_b), climbables.get(*collider_a))
                {
                    climber.intersecting_climbables.insert(climbable);
                };
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_a), climbables.get(*collider_b))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }

                if let (Ok(mut climber), Ok(climbable)) =
                    (climbers.get_mut(*collider_b), climbables.get(*collider_a))
                {
                    climber.intersecting_climbables.remove(&climbable);
                }
            }
        }
    }
}

/// Gravity is multiplied by this scaling factor before it's
/// applied to this [`RigidBody`].
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect)]
#[reflect(Component, PartialEq)]
pub struct GravityScale(pub f32);

impl Default for GravityScale {
    fn default() -> Self {
        Self(1.0)
    }
}

pub fn ignore_gravity_if_climbing(
    mut query: Query<(&Climber, &mut GravityScale), Changed<Climber>>,
) {
    for (climber, mut gravity_scale) in &mut query {
        if climber.climbing {
            gravity_scale.0 = 0.0;
        } else {
            gravity_scale.0 = 1.0;
        }
    }
}
