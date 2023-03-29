#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::{PresentMode, WindowResized},
};

mod effect_renderer;
mod sim_shaders;

use effect_renderer::EffectRenderer;
use sim_shaders::{
    compute_laser_position, compute_ledstrip_position, LaserSim, LaserSimMaterial, LedStripSim,
    LedStripSimMaterial,
};

// TODO: Learn [here](https://github.com/mrk-its/bevy-atari-antic/blob/main/src/render/mod.rs) how to properly integrate this
// using `RenderAssets`. There is a lot of code duplication and stuff currently.

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(EffectRenderer::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "1D Effects Simulator".to_string(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(Material2dPlugin::<LaserSimMaterial>::default())
        .add_plugin(Material2dPlugin::<LedStripSimMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, render_effect_frame)
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

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        LaserSim,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: compute_laser_position(window.width(), window.height()),
            material: laser_materials.add(LaserSimMaterial {}),
            ..default()
        },
    ));

    commands.spawn((
        LedStripSim,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: compute_ledstrip_position(window.width(), window.height()),
            material: ledstrip_materials.add(LedStripSimMaterial {
                effect_data: vec![128],
                widget_resolution: Vec2::new(0., 0.),
            }),
            ..default()
        },
    ));
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut ledstrip_transform: Query<&mut Transform, (With<LedStripSim>, Without<LaserSim>)>,
    mut laser_transform: Query<&mut Transform, (With<LaserSim>, Without<LedStripSim>)>,
) {
    for e in resize_reader.iter() {
        *ledstrip_transform.single_mut() = compute_ledstrip_position(e.width, e.height);
        *laser_transform.single_mut() = compute_laser_position(e.width, e.height);
    }
}

fn render_effect_frame(
    time: Res<Time>,
    mut effect_renderer: ResMut<EffectRenderer>,
    mut led_strips: ResMut<Assets<LedStripSimMaterial>>,
) {
    let mut framebuffer = vec![0u32; 32];
    effect_renderer
        .as_mut()
        .render_next_frame(&mut framebuffer, time.as_ref());

    for (_, material) in led_strips.iter_mut() {
        material.effect_data = framebuffer.to_vec();
    }
}
