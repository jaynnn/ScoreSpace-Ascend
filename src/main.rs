use bevy::prelude::*;

mod input;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::input_plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    commands: &mut Commands
) {
    commands.spawn(Camera2dBundle::default());
}
