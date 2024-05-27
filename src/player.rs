use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::input;
use crate::bullet;
use crate::global;
use crate::wall;
use crate::ground;
use crate::fort;

#[derive(Component)]
pub struct Player {
    isJumping: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            isJumping: false,
        }
    }
    pub fn jump(&mut self) {
        self.isJumping = true;
    }

    pub fn isJumping(&self) -> bool {
        self.isJumping
    }

    pub fn setJumping(&mut self, jumping: bool) {
        println!("setJumping {}", jumping);
        self.isJumping = jumping;
    }
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
        player_jump,
        player_move,
        player_shoot,
        collide_wall,
        collide_ground,
        collide_jump_bullet,
    ));
}

fn spawn_player(
    mut cmds: Commands,
    player_data: Res<PlayerData>
) {
    let input_map = InputMap::new(input::PlayerInputMap::default());
    
    let sprite_size = player_data.sprite_size;
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
        Player::new(),
        InputManagerBundle::with_map(input_map),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn player_jump(
    mut query: Query<(&mut Velocity, &ActionState<input::Action>, &mut Player)>,
    player_data: Res<PlayerData>,
    time: Res<Time>,
    global_data: Res<global::GlobalData>,
) {
    for (mut velocity, action_state, mut player) in query.iter_mut() {
        if action_state.just_pressed(&input::Action::Jump) {
            if !player.isJumping() {
                println!("jump {}", player.isJumping());
                player.setJumping(true);
                velocity.linvel = Vec2::new(0., 1.) * player_data.jump_init_velocity;
            }
        }
        if player.isJumping() {
            if velocity.linvel.y == 0. {
                player.setJumping(false);
            } else {
                let gravity = global_data.gravity;
                velocity.linvel += gravity * time.delta_seconds();
            }
        }
    }
}

fn player_move(
    mut query: Query<(&mut Velocity, &ActionState<input::Action>), With<Player>>,
    player_data: Res<PlayerData>
) {
    for (mut velocity, action_state) in query.iter_mut() {
        let action_press = action_state.get_pressed();
        let mut move_delta = Vec2::ZERO;
        if action_press.len() > 0 {
            let mut x_axis = 0.;
            for action in action_press.iter() {
                match action {
                    input::Action::LeftMove => {
                        x_axis -= 1.;
                    },
                    input::Action::RightMove => {
                        x_axis += 1.;
                    },
                    _ => {},
                }
            }
            if x_axis != 0. {
                move_delta = Vec2::new(x_axis, 0.).normalize();
            }
        }
        velocity.linvel.x = move_delta.x * player_data.move_speed;
    }
}

fn player_shoot(
    mut cmds: Commands,
    mut query: Query<(&ActionState<input::Action>, &Transform, &mut AtkCoolDownTimer), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for (action_state, transform, mut cooldown) in query.iter_mut() {
        cooldown.0.tick(time.delta());
        trace!("cooldown: {:?}", cooldown.0.elapsed().as_secs_f32());
        let atk_pressed = action_state.pressed(&input::Action::LeftShoot);
        let timer_finished = cooldown.0.finished();
        trace!("atk_pressed: {atk_pressed}, timer_finished: {timer_finished}, timer: {:?}", cooldown.0);
        if atk_pressed && timer_finished {
            let direction = get_mouse_direction(transform, point);
            let vel = direction * 1000.;
            trace!("attack {vel:?}");
            let sprite_size = Vec2::splat(8.);
            bullet::spawn_bullet(&mut cmds, bullet::BulletType::Left, transform, vel, sprite_size);
            cooldown.0.reset();
        }
    }
}

// 获取一个给定Transform与鼠标位置的向量
fn get_mouse_direction(
    transform: &Transform,
    cursor_position: Vec2,
) -> Vec2 {
    let cursor_position = Vec2::new(cursor_position.x, cursor_position.y);
    let player_position = transform.translation.xy();
    let direction = cursor_position - player_position;
    direction.normalize()
}

fn collide_wall(
    mut collision_events: EventReader<CollisionEvent>,
    query_player: Query<Entity, With<Player>>,
    query_wall: Query<Entity, With<wall::Wall>>,
    mut player: Query<&mut Player>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(es, ed, _f) => {
                if query_player.contains(*es) || query_player.contains(*ed) {
                    if query_wall.contains(*es) || query_wall.contains(*ed) {
                        let mut player = player.single_mut();
                        player.setJumping(false);
                    }
                
                }
            }
            CollisionEvent::Stopped(es, ed, f) => {

            }
        }

    }
}

fn collide_ground(
    mut collision_events: EventReader<CollisionEvent>,
    query_player: Query<Entity, With<Player>>,
    query_ground: Query<Entity, With<ground::Ground>>,
    mut player: Query<&mut Player>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(es, ed, _f) => {
                if query_player.contains(*es) || query_player.contains(*ed) {
                    if query_ground.contains(*es) || query_ground.contains(*ed) {
                        let mut player = player.single_mut();
                        player.setJumping(false);
                    }
                
                }
            }
            CollisionEvent::Stopped(es, ed, f) => {

            }
        }

    }
}

fn collide_jump_bullet(
    mut query: Query<&mut Player>,
    mut collision_events: EventReader<CollisionEvent>,
    query_player: Query<Entity, With<Player>>,
    query_jump_bullet: Query<Entity, With<fort::JumpBullet>>,
) {
    let mut player = query.single_mut();
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(es, ed, _f) => {
                if query_player.contains(*es) || query_player.contains(*ed) {
                    if query_jump_bullet.contains(*es) || query_jump_bullet.contains(*ed) {
                        player.setJumping(false);
                    }
                }
            }
            CollisionEvent::Stopped(es, ed, f) => {
                if query_player.contains(*es) || query_player.contains(*ed) {
                    if query_jump_bullet.contains(*es) || query_jump_bullet.contains(*ed) {
                        player.setJumping(true);

                    }
                }
            }
        }
    }
}