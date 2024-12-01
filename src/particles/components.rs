use bevy::prelude::*;
use bevy::utils::HashMap;
use std::hash::Hash;

/// Component for particles
#[derive(Component)]
pub struct Particle {
    pub p: Vec3,
    pub v: Vec3,
}

pub struct HashGrid {
    pub particles: Vec<(u32, usize)>,
    pub grid: HashMap<u32, usize>,
}

#[derive(Hash)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}
