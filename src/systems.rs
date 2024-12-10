use super::components::*;
use bevy::{
    color::palettes::css::*,
    math::prelude::*,
    pbr::wireframe::{Wireframe, WireframeColor},
    prelude::*,
    render::{mesh::MeshAabb, primitives::Aabb},
};

pub const BOUNDINGBOX_SIZE: f32 = 5.0;

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set background color
    commands.insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.8)));

    // bounding box
    let min = Vec3::ONE * -BOUNDINGBOX_SIZE / 2.0;
    let max = Vec3::ONE * BOUNDINGBOX_SIZE / 2.0;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_corners(min, max))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 0.0), // Red color with 50% transparency
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        })),
        Aabb::from_min_max(min, max),
        Wireframe,
        WireframeColor { color: RED.into() },
        BoundingBox,
    ));
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::from_corners(min, max))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.0, 1.0, 0.0), // Green color with 50% transparency
    //         alpha_mode: AlphaMode::Blend,
    //         ..Default::default()
    //     })),
    //     Wireframe,
    //     WireframeColor { color: GREEN.into() },
    // ))

    // light
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 10000.0,
            range: 100.0,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
