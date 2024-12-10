use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::utils::HashMap;
use rand::Rng;

use super::components::*;
use crate::components::BoundingBox;
use crate::systems::BOUNDINGBOX_SIZE;

pub const PARTICLE_COUNT: usize = 1000;
pub const PARTICLE_SIZE: f32 = 0.03;
pub const SMOOTHING_LENGTH: f32 = 0.1;
pub const GRID_SIZE: f32 = SMOOTHING_LENGTH;

pub fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let half_box = BOUNDINGBOX_SIZE / 2.0;
    let range = -half_box..half_box;
    for _ in 0..PARTICLE_COUNT {
        let position = Vec3 {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range.clone()),
        };

        commands.spawn((
            Particle {
                p: position,
                v: Vec3::ZERO,
            },
            Mesh3d(meshes.add(Sphere::new(PARTICLE_SIZE))),
            MeshMaterial3d(materials.add(Color::from(BLUE))),
            Transform::from_translation(position),
        ));
    }
}

fn hash_cell(c: Cell) -> u32 {
    let mut hasher = DefaultHasher::new();
    c.hash(&mut hasher);
    hasher.finish() as u32 % PARTICLE_COUNT as u32
}

fn find_cell(p: Vec3) -> Cell {
    let x = (p.x / GRID_SIZE).floor() as u32;
    let y = (p.y / GRID_SIZE).floor() as u32;
    Cell { x: x, y: y }
}

// pub fn calculate_neighbors(particles: &Vec<Mut<'_, Particle>>) -> HashGrid {
//     let mut particle_cell = Vec::new();
//     let mut hash_start = HashMap::new();
//     for i in 0..particles.len() {
//         let p = &particles[i];
//         let cell = find_cell(p.p);
//         let hash = hash_cell(cell);
//         particle_cell.push((hash, i));
//     }

//     particle_cell.sort_by(|a, b| a.0.cmp(&b.0));

//     let mut cell_idx = 0;
//     for (i, (j, p)) in particle_cell.into_iter().enumerate() {
//         while j > cell_idx {
//             hash_start.insert(cell_idx, particle_cell.len());
//             cell_idx += 1;
//         }
//         if j == cell_idx {
//             hash_start.insert(cell_idx, i);
//             cell_idx += 1;
//         }
//     }

//     HashGrid {
//         particles: particle_cell,
//         grid: hash_start,
//     }
// }

// fn find_neighbors(
//     p: usize,
//     particles: &Vec<Mut<'_, Particle>>,
//     hash_grid: &HashGrid,
// ) -> Vec<usize> {
//     let cell = find_cell(p.p);
//     let hash = hash_cell(cell);
//     let i = hash_grid.grid.get(&hash).unwrap();

//     let mut neighbors = Vec::new();
//     while hash_grid.particles[*i].0 == hash {
//         let j = hash_grid.particles[*i].1;
//         if j != p {
//             neighbors.push(j);
//         }
//     }

//     neighbors
// }

pub fn step(
    mut query_particles: Query<&mut Particle>,
    query_bounding_box: Query<&Aabb, With<BoundingBox>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d>,
    time: Res<Time>,
) {
    let Ok(bbox) = query_bounding_box.get_single() else {
        return;
    };

    let particles = query_particles.iter_mut().collect::<Vec<_>>();

    let mesh_handle = query.get_single().expect("Query not successful");
    let mesh = meshes.get_mut(mesh_handle).unwrap();

    // calculate_neighbors(&particles);
    for mut p in particles {
        let mut v = p.v;
        v.y -= 9.81 * time.delta_secs();
        v *= 0.99;
        p.v = v;
        p.p += v * time.delta_secs();

        let min = bbox.min();
        let max = bbox.max();

        for i in 0..3 {
            if p.p[i] < min[i] {
                p.p[i] = min[i];
                p.v[i] *= -1.0;
            }
            if p.p[i] > max[i] {
                p.p[i] = max[i];
                p.v[i] *= -1.0;
            }
        }
    }
}

pub fn move_spheres(mut query: Query<(&mut Transform, &Particle)>) {
    for (mut transform, p) in query.iter_mut() {
        transform.translation = p.p;
    }
}
