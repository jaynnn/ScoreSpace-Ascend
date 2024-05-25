

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn input_plugin(app: &mut App) {
    app
    .add_plugins(InputManagerPlugin::<Action>::default());
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Actionlike, Reflect)]
pub enum Action {
    Jump,
    LeftMove,
    RightMove,
    LeftShoot,
}

const PLAYER_INPUT_KEY_COUNT: usize = 4;
const PLAYER_INPUT_MAP_1: [(Action, InputKind); PLAYER_INPUT_KEY_COUNT]  = [
    (Action::Jump, InputKind::PhysicalKey(KeyCode::Space)),
    (Action::LeftMove, InputKind::PhysicalKey(KeyCode::KeyA)),
    (Action::RightMove, InputKind::PhysicalKey(KeyCode::KeyD)),
    (Action::LeftShoot, InputKind::Mouse(MouseButton::Left)),
];

pub struct PlayerInputMap {
    map: Vec<(Action, InputKind)>,
}

impl Default for PlayerInputMap {
    fn default() -> Self {
        let map = Vec::from(PLAYER_INPUT_MAP_1);
        Self {
            map
        }
    }
}

impl IntoIterator for PlayerInputMap {
    type Item = (Action, InputKind);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}