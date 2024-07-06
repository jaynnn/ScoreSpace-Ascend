// 道具轮盘

use bevy::prelude::*;

use crate::player::Player;
use crate::global::GlobalData;

pub fn roulette_plugin(app: &mut App) {
    app
    
        .add_systems(PreUpdate, (
            roulette_event,
        ))
        .add_systems(Update, (
            setup,
            update_roulette,
        ));
}

// 轮盘物品
#[derive(Component)]
pub struct RouletteItem {
    pub name: String,
    pub sprite_bundle: SpriteBundle,
}

// 轮盘
#[derive(Component)]
pub struct Roulette {
    list: Vec<RouletteItem>,
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

    pub fn add_item(&mut self, item: RouletteItem) {
        self.list.push(item);
    }

    pub fn get_cur_item(&self) -> Option<&RouletteItem> {
        self.list.get(self.cur_index)
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

    cmds.spawn((roulette,));
}

#[derive(Event)]
pub enum RouletteEvent {
    Left,
    Right
}

// 轮盘事件, 切换物品
// 用法：触发RouletteEvent事件
pub fn roulette_event(
    mut roulette_events: EventReader<RouletteEvent>,
    mut query: Query<&mut Roulette>,
) {
    for event in roulette_events.read() {
        for mut roulette in query.iter_mut() {
            match event {
                RouletteEvent::Left => {
                    roulette.prev();
                }
                RouletteEvent::Right => {
                    roulette.next();
                }
            }
        }
    }
}

pub fn update_roulette(
    query_player: Query<&Transform, With<Player>>,
    mut query_roulette: Query<(&Roulette, &mut Transform)>,
    global_data: Res<GlobalData>,
) {
    let player_pos = query_player.single();
    for (roulette, mut roulette_pos) in query_roulette.iter_mut() {

        let cur_roulette_pos = player_pos.translation.y + 200. * global_data.scale;
        roulette_pos.translation.x = player_pos.translation.x;
        roulette_pos.translation.y = cur_roulette_pos;

        
    }
}