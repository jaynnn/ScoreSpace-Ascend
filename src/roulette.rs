// 道具轮盘

use bevy::ecs::event;
use bevy::{prelude::*, transform};

use crate::player::Player;
use crate::global::{self, GlobalData};

pub fn roulette_plugin(app: &mut App) {
    app
        .register_type::<Roulette>()
        .add_event::<RouletteRotateEvent>()
        .add_event::<RouletteItemAddEvent>()
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(PreUpdate, (
            roulette_event,
        ))
        .add_systems(Update, (
            // update_roulette,
            on_add_item,
            test_add_item,
            test_rotate,
        ));
}

// 轮盘物品
#[derive(Component)]
pub struct RouletteItem {
    pub id: u32,
    pub name: String,
}

// 轮盘
#[derive(Component, Reflect)]
pub struct Roulette {
    list: Vec<Entity>,
    cur_index: usize,
}

impl Default for Roulette {
    fn default() -> Self {
        Self {
            list: vec![],
            cur_index: 0,
        }
    }
}

impl Roulette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn add_item(&mut self, item: Entity) {
        self.list.push(item);
    }

    pub fn get_item(&self, index: usize) -> Option<&Entity> {
        self.list.get(index)
    }

    pub fn get_cur_index(&self) -> usize {
        self.cur_index
    }

    pub fn get_cur_item(&self) -> Option<&Entity> {
        self.list.get(self.cur_index)
    }

    pub fn get_prev_item(&self) -> Option<&Entity> {
        self.list.get((self.cur_index + self.list.len() - 1) % self.list.len())
    }

    pub fn get_next_item(&self) -> Option<&Entity> {
        self.list.get((self.cur_index + 1) % self.list.len())
    }

    // 切换到下一个物品
    pub fn next(&mut self) {
        self.cur_index = (self.cur_index + 1) % self.list.len();
    }

    // 切换到上一个物品
    pub fn prev(&mut self) {
        self.cur_index = (self.cur_index + self.list.len() - 1) % self.list.len();
    }
}

fn setup(
    mut cmds: Commands,
) {
    let roulette = Roulette::new();

    cmds.spawn((
        roulette,
        Name::new("Roulette")
    ));
}

#[derive(Event)]
pub enum RouletteRotateEvent {
    Left,
    Right
}

// 轮盘事件, 切换物品
// 用法：触发RouletteEvent事件
pub fn roulette_event(
    mut roulette_events: EventReader<RouletteRotateEvent>,
    mut query_roulette_item: Query<(&mut Transform, &mut RouletteItem)>,
    mut query: Query<&mut Roulette>,
    global_data: Res<GlobalData>,
) {
    for event in roulette_events.read() {
        for mut roulette in query.iter_mut() {
            match event {
                RouletteRotateEvent::Left => {
                    roulette.prev();
                }
                RouletteRotateEvent::Right => {
                    roulette.next();
                }
            }
        }
        
        let roulette = query.single();
        println!("cur_index: {} {}", roulette.get_cur_index(), roulette.len());
        let cur_index = roulette.get_cur_index();
        for i in 0..roulette.len() {
            let item_id = roulette.get_item(i).unwrap();
            if let Ok((mut transform, mut item)) = query_roulette_item.get_mut(*item_id) {
                println!("======, {} {}", i, cur_index);
                transform.translation.x = (i as f32 - cur_index as f32) * 50. * global_data.scale;
            }
        }
    }
}

pub fn test_rotate(
    mut events: EventWriter<RouletteRotateEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        events.send(RouletteRotateEvent::Left);
    }
    if input.just_pressed(KeyCode::KeyE) {
        events.send(RouletteRotateEvent::Right);
    }
}

#[derive(Event)]
pub struct RouletteItemAddEvent {
    pub id: u32,
}

// 添加物品
pub fn on_add_item(
    mut cmds: Commands,
    mut roulette_item_add_events: EventReader<RouletteItemAddEvent>,
    mut query_roulette: Query<&mut Roulette>,
    query_player_e: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    global_data: Res<GlobalData>,
) {
    if let Ok(player) = query_player_e.get_single()
    {
        let mut roulette = query_roulette.single_mut();
        for event in roulette_item_add_events.read() {
            cmds.entity(player).with_children(|parent| {
                let roulette_len = roulette.len();
                let cur_index = roulette.get_cur_index();
                let transform_x = (roulette_len - cur_index) as f32 * 50. * global_data.scale;
                let id = parent.spawn((
                    RouletteItem {
                        id: event.id,
                        name: "test".to_string(),
                    }, SpriteBundle {
                        sprite: Sprite {
                            ..default()
                        },
                        texture: asset_server.load("images/player.png"),
                        transform: Transform {
                            translation: Vec3::new(transform_x, 50., 0.),
                            ..default()
                        },
                        ..default()
                    },
                )).id();
                roulette.add_item(id);
            });
        }
    }
}

pub fn test_add_item(
    mut events: EventWriter<RouletteItemAddEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyI) {
        events.send(RouletteItemAddEvent {
            id: 1,
        });
    }
}
