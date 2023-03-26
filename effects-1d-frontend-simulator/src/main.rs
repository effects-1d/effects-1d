#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

mod materials;
use materials::{LaserSimMaterial, LedStripSimMaterial};

#[derive(Component)]
struct LaserSim;

#[derive(Component)]
struct LedStripSim;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<LaserSimMaterial>::default())
        .add_plugin(Material2dPlugin::<LedStripSimMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .run();
}

fn led_strip_position(window_width: f32, window_height: f32) -> Transform {
    Transform::default()
        .with_scale(Vec3::new(window_width, window_height * 0.05, 0.))
        .with_translation(Vec3::new(0., window_height * (0.5 - 0.05), 0.))
}

fn laser_position(window_width: f32, window_height: f32) -> Transform {
    Transform::default()
        .with_scale(Vec3::new(window_width, window_height * 0.9, 0.))
        .with_translation(Vec3::new(0., -window_height * 0.05, 0.))
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

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        LaserSim,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: laser_position(window.width(), window.height()),
            material: laser_materials.add(LaserSimMaterial {}),
            ..default()
        },
    ));
    commands.spawn((
        LedStripSim,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: led_strip_position(window.width(), window.height()),
            material: ledstrip_materials.add(LedStripSimMaterial {}),
            ..default()
        },
    ));
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut ledstrip: Query<&mut Transform, (With<LedStripSim>, Without<LaserSim>)>,
    mut laser: Query<&mut Transform, (With<LaserSim>, Without<LedStripSim>)>,
) {
    for e in resize_reader.iter() {
        *ledstrip.single_mut() = led_strip_position(e.width, e.height);
        *laser.single_mut() = laser_position(e.width, e.height);
    }
}
