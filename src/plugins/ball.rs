use std::f32::consts::PI;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        system::{Commands, Query, ResMut},
    },
    math::primitives::Circle,
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
};

use crate::plugins::position::RadialPosition;

pub struct BallPlugin;

#[derive(Resource, Default)]
pub struct MaxBallID(i32);

#[derive(Component)]
#[require(RadialPosition)]
pub struct Ball {
    pub id: i32,
}

// Plugin root
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaxBallID(0))
            .add_systems(FixedUpdate, distribute_balls);
    }
}

impl Ball {
    pub fn spawn_ball(
        mut max_id: ResMut<MaxBallID>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        const BALL_SIZE: f32 = 5.0;
        let mesh = meshes.add(Circle::new(BALL_SIZE));
        let material = materials.add(Color::srgb(1.0, 0.0, 0.0));

        // Ensure ID stays unique
        max_id.0 += 1;

        let ball = commands.spawn((
            Self { id: max_id.0 },
            RadialPosition {
                radius: 50.0,
                ..Default::default()
            },
            Mesh2d(mesh),
            MeshMaterial2d(material),
        ));
    }
}

fn distribute_balls(mut balls: Query<&mut RadialPosition, With<Ball>>) {
    let angular_gap = (2. * PI) / ((balls.iter().len()) as f32);
    let mut i = 0;

    for mut radial_position in &mut balls {
        i += 1;
        radial_position.angle = i as f32 * angular_gap;
    }
}
