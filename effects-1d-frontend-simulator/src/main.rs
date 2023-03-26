#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<LaserSimMaterial>::default())
        .add_plugin(Material2dPlugin::<LedStripSimMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    windows: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut laser_materials: ResMut<Assets<LaserSimMaterial>>,
    mut ledstrip_materials: ResMut<Assets<LedStripSimMaterial>>,
) {
    // Assumes we only have one window
    let window = windows.single();

    // TODO: change transform/scale when window size changes

    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_scale(Vec3::new(window.width() as f32, window.height() * 0.9, 0.))
            .with_translation(Vec3::new(0., -window.height() * 0.05, 0.)),
        material: laser_materials.add(LaserSimMaterial {}),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_scale(Vec3::new(window.width() as f32, window.height() * 0.05, 0.))
            .with_translation(Vec3::new(0., window.height() * (0.5 - 0.05), 0.)),
        material: ledstrip_materials.add(LedStripSimMaterial {}),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a71be379-cac5-4204-8dbd-33982a9e1a60"]
struct LaserSimMaterial {}

impl Material2d for LaserSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/laser.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "69fdd51f-11e6-4ffd-9ebf-2badfdd98f37"]
struct LedStripSimMaterial {}

impl Material2d for LedStripSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/led_strip.wgsl".into()
    }
}
