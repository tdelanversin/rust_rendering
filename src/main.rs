mod components;
mod particles;
mod systems;

use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use particles::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin,
        ))
        .add_plugins(ParticlePlugin)
        .add_systems(Startup, setup)
        .run();
}
