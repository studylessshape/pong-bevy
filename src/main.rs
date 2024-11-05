use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Update, project_position)
        .run();
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::ZERO),
        }
    }
}

fn spawn_ball(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(BallBundle::new());
}

fn project_position(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in positionables.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
