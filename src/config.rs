use bevy::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};

use crate::roulette::RouletteItemInfo;

pub fn config_plugin(app: &mut App) {
    app
        .add_plugins((
            CsvAssetPlugin::<RouletteItemInfo>::new(&["config/item.csv"]),
        ))
        .add_systems(Startup, setup)
        ;
}


#[derive(Resource)]
pub struct ItemsHandle(Handle<LoadedCsv<RouletteItemInfo>>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let items: ItemsHandle = ItemsHandle(asset_server.load("config/item.csv"));
    commands.insert_resource(items);
}