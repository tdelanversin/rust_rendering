mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_particles);
        app.add_systems(Update, (step, move_spheres));
    }
}
