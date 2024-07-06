use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::wall::WallBundle;
use crate::scene::LadderBundle;
use crate::player::PlayerBundle;
use crate::scene::MobBundle;
use crate::scene::ChestBundle;
use crate::scene::PumpkinsBundle;


pub fn ldtk_plugin(app: &mut App) {
    app
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<LadderBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<MobBundle>("Mob")
        .register_ldtk_entity::<ChestBundle>("Chest")
        .register_ldtk_entity::<PumpkinsBundle>("Pumpkins");
}