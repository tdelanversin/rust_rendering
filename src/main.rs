mod components;
mod particles;
mod systems;

use bevy::prelude::*;

use particles::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ParticlePlugin)
        .add_systems(Startup, setup)
        .run();
}
