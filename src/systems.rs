use bevy::math::prelude::*;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use super::components::*;

pub const BOUNDINGBOX_SIZE: f32 = 5.0;

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set background color
    commands.insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.8)));

    // bounding box
    let min = Vec3::ONE * -BOUNDINGBOX_SIZE / 2.0;
    let max = Vec3::ONE * BOUNDINGBOX_SIZE / 2.0;
    commands.spawn((Aabb::from_min_max(min, max), BoundingBox));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
