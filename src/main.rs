use bevy::{
    app::{App, FixedUpdate, Startup},
    camera::Camera2d,
    ecs::{
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query},
    },
    input::{
        common_conditions::{input_just_pressed, input_toggle_active},
        keyboard::KeyCode,
    },
    DefaultPlugins,
};

use crate::plugins::{
    ball::{Ball, BallPlugin},
    interface::InterfacePlugin,
    position::{PositionPlugin, RadialPosition},
};

mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PositionPlugin, BallPlugin, InterfacePlugin))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            Ball::spawn_ball.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(
            FixedUpdate,
            increase_ball_1_r.run_if(input_just_pressed(KeyCode::KeyW)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn increase_ball_1_r(mut balls: Query<(&Ball, &mut RadialPosition), With<Ball>>) {
    for (ball, mut radial_position) in &mut balls {
        if ball.id == 1 {
            radial_position.radius += 10.0;
        }
    }
}
