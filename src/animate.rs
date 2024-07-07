use bevy::transform;
use bevy::{prelude::*, render::render_resource::Texture};

use crate::player::Player;
use crate::comm::SetUpFlag;

pub fn animate_plugin(app: &mut App) {
    app
        .add_event::<PlayerAnimateEvent>()
        .add_systems(Update, (
            setup,
            player_animate,
        ))
        ;
}



#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Default, Clone)]
pub struct PlayerAnimateIndices {
    run: AnimationIndices,
    walk: AnimationIndices,
    jump: AnimationIndices,
    die: AnimationIndices,
    idle: AnimationIndices,
}

pub fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query_player: Query<(Entity, &mut Transform), With<Player>>,
    mut local_data: Local<SetUpFlag>,
) {
    if local_data.0 {
        return;
    }
    for (player_e, transform) in query_player.iter_mut() {
        let texture: Handle<Image>  = asset_server.load("atlas/run_walk_2.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(245.0, 305.0), 4, 2, None, None);
        let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
        let animation_indices = PlayerAnimateIndices {
            run: AnimationIndices { first: 0, last: 3 },
            walk: AnimationIndices { first: 4, last: 7 },
            idle: AnimationIndices { first: 4, last: 4},
            ..default()
        };
        cmds.entity(player_e)
            .insert(animation_indices.clone())
            .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .insert(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_indices.walk.first,
            })
            .insert(texture)
            .insert(transform.with_scale(Vec3::splat(0.3)));
        local_data.0 = true;
    }
}

#[derive(Event)]
pub enum PlayerAnimateEvent {
    Run(Vec2),
    Walk(Vec2),
    Jump(Vec2),
    Die(Vec2),
    Idle(Vec2),
}

pub fn player_animate(
    mut evt: EventReader<PlayerAnimateEvent>,
    mut query: Query<(&PlayerAnimateIndices, &mut AnimationTimer, &mut TextureAtlas, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    for e in evt.read() {
        let (player_animate_indices, mut animate_timer, mut player_atlas, mut sprie) = query.single_mut();
        animate_timer.tick(time.delta());
        if animate_timer.just_finished() {
            match e {
                PlayerAnimateEvent::Run(vel) => {
                    if player_animate_indices.run.first <= player_atlas.index && player_atlas.index <= player_animate_indices.run.last {
                        player_atlas.index = if player_atlas.index == player_animate_indices.run.last {
                            player_animate_indices.run.first
                        } else {
                            player_atlas.index + 1
                        };
                        
                    } else {
                        player_atlas.index = player_animate_indices.run.first;
                    }
                    println!("Run");
                }
                PlayerAnimateEvent::Walk(vel) => {
                    if player_animate_indices.walk.first <= player_atlas.index && player_atlas.index <= player_animate_indices.walk.last {
                        player_atlas.index = if player_atlas.index == player_animate_indices.walk.last {
                            player_animate_indices.walk.first
                        } else {
                            player_atlas.index + 1
                        };
                    } else {
                        player_atlas.index = player_animate_indices.walk.first;
                    }
                    if vel.x < 0. {
                        sprie.flip_x = true;
                    } else {
                        sprie.flip_x = false;
                    }
                    println!("Walk {}", player_atlas.index);
                }
                PlayerAnimateEvent::Jump(vel) => {
                    println!("Jump");
                }
                PlayerAnimateEvent::Die(vel) => {
                    println!("Die");
                }
                PlayerAnimateEvent::Idle(vel) => {
                    player_atlas.index = player_animate_indices.idle.first;
                    println!("Idle");
                }
            }
        }
    }
}