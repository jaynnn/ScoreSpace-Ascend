// 道具轮盘

use bevy::ecs::event;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::player::Player;
use crate::global::GlobalData;

pub fn roulette_plugin(app: &mut App) {
    app
        .register_type::<Roulette>()
        .add_event::<RouletteRotateEvent>()
        .add_event::<RouletteItemAddEvent>()
        .add_event::<ShowItemEvent>()
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(PreUpdate, (
            roulette_event,
        ))
        .add_systems(Update, (
            // update_roulette,
            on_add_item,
            timer_hide_item,
            show_all_item,

            test_add_item,
            test_rotate,
        ));
}

// 轮盘物品
#[derive(Component)]
pub struct RouletteItem {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, TypePath, Asset, Clone)]
pub struct RouletteItemInfo {
    pub id: u32,
    pub name: String,
    pub texture: String,
    pub width: f32,
    pub height: f32,
}

// 轮盘
#[derive(Component, Reflect)]
pub struct Roulette {
    list: Vec<Entity>,
    cur_index: usize,
    timer: Timer,
}

impl Default for Roulette {
    fn default() -> Self {
        Self {
            list: vec![],
            cur_index: 0,
            timer: Timer::from_seconds(2., TimerMode::Repeating),
        }
    }
}

impl Roulette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_all_items(&self) -> &Vec<Entity> {
        &self.list
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
        if self.list.len() > 0 {
            self.cur_index = (self.cur_index + 1) % self.list.len();
        }
    }

    // 切换到上一个物品
    pub fn prev(&mut self) {
        if self.list.len() > 0 {
            self.cur_index = (self.cur_index + self.list.len() - 1) % self.list.len();
        }
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
    mut show_event: EventWriter<ShowItemEvent>,
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
        
        let mut roulette = query.single_mut();
        roulette.timer.reset();
        println!("cur_index: {} {}", roulette.get_cur_index(), roulette.len());
        let cur_index = roulette.get_cur_index();
        for i in 0..roulette.len() {
            let item_id = roulette.get_item(i).unwrap();
            if let Ok((mut transform, mut item)) = query_roulette_item.get_mut(*item_id) {
                // println!("======, {} {}", i, cur_index);
                transform.translation.x = (i as f32 - cur_index as f32) * 50. * global_data.scale;
            }
        }
        show_event.send(ShowItemEvent::Three);
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
    mut show_event: EventWriter<ShowItemEvent>,
    item_infos: Res<Assets<RouletteItemInfo>>,
) {
    if let Ok(player) = query_player_e.get_single()
    {
        let mut roulette = query_roulette.single_mut();
        for event in roulette_item_add_events.read() {
            cmds.entity(player).with_children(|parent| {
                let roulette_len = roulette.len();
                let cur_index = roulette.get_cur_index();
                let transform_x = (roulette_len - cur_index) as f32 * 50. * global_data.scale;
                // let item_info = item_infos.get(&event.id).unwrap();
                let id = parent.spawn((
                    RouletteItem {
                        id: event.id,
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
                roulette.timer.reset();
                show_event.send(ShowItemEvent::All);
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

pub fn timer_hide_item(
    time: Res<Time>,
    mut query_roulette: Query<&mut Roulette>,
    mut query_roulette_item: Query<(&mut Visibility, &RouletteItem)>,
) {
    let mut roulette = query_roulette.single_mut();
    roulette.timer.tick(time.delta());
    
    if roulette.timer.finished() {
        if let Some(cur_item_id) = roulette.get_cur_item() {
            for item_id in roulette.get_all_items() {
                if let Ok((mut visibility, _)) = query_roulette_item.get_mut(*item_id) {
                    if item_id != cur_item_id {
                        // println!("hide item {:?}", item_id);
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}


#[derive(Event)]
pub enum ShowItemEvent {
    All,
    Three,
}

pub fn show_all_item(
    mut events: EventReader<ShowItemEvent>,
    roulette: Query<&Roulette>,
    mut query_roulette_item: Query<(&mut Visibility, &RouletteItem)>,
) {
    for evt in events.read() {
        let roulette = roulette.single();
        match evt {
            ShowItemEvent::All => {
                for item_id in roulette.get_all_items() {
                    if let Ok((mut visibility, _)) = query_roulette_item.get_mut(*item_id) {
                        *visibility = Visibility::Visible;
                    }
                }
            }
            ShowItemEvent::Three => {
                let cur_index = roulette.get_cur_index();
                let pre_index = cur_index as i32 - 1;
                let next_index = cur_index + 1;
                println!("===== {} {} {}", pre_index, cur_index, next_index);
                for i in 0..roulette.len() {
                    let item_id = roulette.get_item(i).unwrap();
                    if let Ok((mut visibility, _)) = query_roulette_item.get_mut(*item_id) {
                        if (pre_index >= 0 && i as i32 == pre_index) || i == cur_index || (next_index < roulette.len() && i == next_index) {
                            *visibility = Visibility::Visible;
                        } else {
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
        }
    }
}