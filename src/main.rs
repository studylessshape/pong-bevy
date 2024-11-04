use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ball)
        .run();
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn_empty();
}
