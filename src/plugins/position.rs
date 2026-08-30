use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{component::Component, system::Query},
    math::Vec2,
    transform::components::Transform,
};

pub struct PositionPlugin;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(Vec2);

#[derive(Component, Default)]
#[require(Position)]
pub struct RadialPosition {
    pub centre: Vec2,
    pub radius: f32,
    pub angle: f32,
}

// Plugin root
impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (project_positions, project_radial_positions));
    }
}

/// Call on `FixedUpdate` to keep Bevy's `Transform` position up to date with out `Position` position
fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

/// Keep all `RadialPosition`s up to date with some basic trig
fn project_radial_positions(mut positionables: Query<(&mut Position, &RadialPosition)>) {
    for (mut position, radial_position) in &mut positionables {
        position.0.x =
            radial_position.centre.x + radial_position.angle.sin() * radial_position.radius;

        position.0.y =
            radial_position.centre.y + radial_position.angle.cos() * radial_position.radius;
    }
}
