use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy::utils::HashSet;

use crate::input;
use crate::global;
use crate::wall;


use crate::scene::Climbable;
use crate::scene::ColliderBundle;
use crate::scene::Items;

pub fn player_plugin(app: &mut App) {
    app
    .insert_resource(PlayerData { 
        jump_init_velocity: 1000.,
        move_speed: 200.,
        sprite_size: Vec2::splat(20.),
    })
    .register_type::<PlayerData>()
    .add_plugins(ResourceInspectorPlugin::<PlayerData>::default())
    .add_systems(Startup, (
        spawn_player,
    ))
    .add_systems(Update, (
        player_move,
        // player_shoot,
        detect_climb_range,
        ignore_gravity_if_climbing,
    ));
}

#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("images/player.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,

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

#[derive(Component)]
pub struct AtkCoolDownTimer(Timer);

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlayerData {
    pub jump_init_velocity: f32,
    pub move_speed: f32,
    pub sprite_size: Vec2,
}

pub fn player_plugin(app: &mut App) {
    app
    .insert_resource(PlayerData { 
        jump_init_velocity: 1000.,
        move_speed: 200.,
        sprite_size: Vec2::splat(20.),
    })
    .register_type::<PlayerData>()
    .add_plugins(ResourceInspectorPlugin::<PlayerData>::default())
    .add_systems(Startup, (
        spawn_player,
    ))
    .add_systems(Update, (
        player_move,
        detect_climb_range,
        ignore_gravity_if_climbing,
    ));
}

fn spawn_player(
    mut cmds: Commands,
    player_data: Res<PlayerData>
) {
}

fn player_move(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>,
) {
    for (mut velocity, mut climber, ground_detection) in &mut query {
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
    }
}

fn get_mouse_direction(
    transform: &Transform,
    cursor_position: Vec2,
) -> Vec2 {
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