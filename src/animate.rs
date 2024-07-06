use bevy::{prelude::*, render::render_resource::Texture};

use crate::player::Player;

pub fn animate_plugin(app: &mut App) {
    app
        .add_event::<PlayerAnimateEvent>()
        // .add_systems(Startup, (
        //     setup.after(crate::main_setup),
        // ))
        // .add_systems(Update, (
        //     player_animate,
        // ))
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
    mut query_player: Query<(Entity, &mut TextureAtlas, &mut Handle<Image>), With<Player>>
) {
    let texture: Handle<Image>  = asset_server.load("atlas/run_walk.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
    let animation_indices = PlayerAnimateIndices {
        run: AnimationIndices { first: 1, last: 4 },
        walk: AnimationIndices { first: 5, last: 8 },
        ..default()
    };
    let (player_e, mut player_atlas, mut player_texture) = query_player.single_mut();
    cmds.entity(player_e)
        .insert(animation_indices.clone())
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Once)));
    *player_texture = texture;
    player_atlas.layout = texture_atlas_layout;
    player_atlas.index = animation_indices.walk.first;
}

#[derive(Event)]
pub enum PlayerAnimateEvent {
    Run,
    Walk,
    Jump,
    Die,
    Idle,
}

pub fn player_animate(
    mut evt: EventReader<PlayerAnimateEvent>,
    mut query: Query<(&PlayerAnimateIndices, &mut AnimationTimer, &mut TextureAtlas), With<Player>>,
    time: Res<Time>,
) {
    for e in evt.read() {
        let (player_animate_indices, mut animate_timer, mut player_atlas) = query.single_mut();
        animate_timer.tick(time.delta());
        if animate_timer.just_finished() {
            match e {
                PlayerAnimateEvent::Run => {
                    if player_animate_indices.run.first <= player_atlas.index && player_atlas.index <= player_animate_indices.run.last {
                        player_atlas.index = if player_atlas.index == player_animate_indices.run.last {
                            player_animate_indices.run.first
                        } else {
                            player_atlas.index + 1
                        };
                        
                    } else {
                        player_atlas.index += 1;
                    }
                    println!("Run");
                }
                PlayerAnimateEvent::Walk => {
                    if player_animate_indices.walk.first <= player_atlas.index && player_atlas.index <= player_animate_indices.walk.last {
                        player_atlas.index = if player_atlas.index == player_animate_indices.walk.last {
                            player_animate_indices.walk.first
                        } else {
                            player_atlas.index + 1
                        };
                    } else {
                        player_atlas.index += 1;
                    }
                    println!("Walk");
                }
                PlayerAnimateEvent::Jump => {
                    println!("Jump");
                }
                PlayerAnimateEvent::Die => {
                    println!("Die");
                }
                PlayerAnimateEvent::Idle => {
                    println!("Idle");
                }
            }
        }
    }
}