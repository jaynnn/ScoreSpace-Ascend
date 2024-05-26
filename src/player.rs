use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::input;
use crate::bullet;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct AtkCoolDownTimer(Timer);

pub fn player_plugin(app: &mut App) {
    app
    .add_systems(Startup, (
        spawn_player,
    ))
    .add_systems(Update, (
        player_jump,
        player_move,
        player_shoot,
    ));
}

fn spawn_player(
    mut cmds: Commands,

) {
    let input_map = InputMap::new(input::PlayerInputMap::default());
    
    let sprite_size = Vec2::splat(16.);
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
        Player,
        InputManagerBundle::with_map(input_map),
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::cuboid(sprite_size.x/2., sprite_size.y/2.),
    ));
}

fn player_jump(
    mut query: Query<(&mut Velocity, &ActionState<input::Action>), With<Player>>,
) {
    for (mut velocity, action_state) in query.iter_mut() {
        if action_state.just_pressed(&input::Action::Jump) {
            velocity.linvel = Vec2::new(0., 1.) * 100.;
        }
    }
}

fn player_move(
    mut query: Query<(&mut Velocity, &ActionState<input::Action>), With<Player>>,
) {
    for (mut velocity, action_state) in query.iter_mut() {
        let mut x = 0.;
        let action_press = action_state.get_pressed();
        if action_state.pressed(&input::Action::LeftMove) {
            x -= 1.;
        }
        if action_state.pressed(&input::Action::RightMove) {
            x += 1.;
        }
        let mut move_delta = Vec2::ZERO;
        if action_press.len() > 0 {
            let mut x_axis = 0.;
            let mut y_axis = 0.;
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
            if x_axis != 0. && y_axis != 0. {
                move_delta = Vec2::new(x_axis, y_axis).normalize();
                println!("player_move {:?}", move_delta);
            }
        }
        velocity.linvel = move_delta * 100.;
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
