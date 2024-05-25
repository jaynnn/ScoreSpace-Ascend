

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