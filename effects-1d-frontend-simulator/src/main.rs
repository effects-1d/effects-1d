#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            encase::StorageBuffer, Buffer, BufferDescriptor, BufferInitDescriptor, BufferUsages,
        },
        renderer::{RenderDevice, RenderQueue},
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::{PresentMode, WindowResized},
};

mod sim_shaders;
use sim_shaders::{
    laser_position, led_strip_position, LaserSim, LaserSimMaterial, LedStripSim,
    LedStripSimMaterial,
};

// TODO: Learn [here](https://github.com/mrk-its/bevy-atari-antic/blob/main/src/render/mod.rs) how to modify buffers at runtime.
// Might need to make the sim renderers `RenderAssets`.

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
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
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(Material2dPlugin::<LaserSimMaterial>::default())
        .add_plugin(Material2dPlugin::<LedStripSimMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, update_effect)
        .run();
}

fn setup(
    windows: Query<&Window>,
    render_device: Res<RenderDevice>,
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

    let mut buffer_data = StorageBuffer::new(Vec::new());
    buffer_data
        .write(&[1, 50, 255, 0, 18, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("effect data buffer"),
        usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        contents: buffer_data.as_ref(),
    });

    commands.spawn((
        LedStripSim,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: led_strip_position(window.width(), window.height()),
            material: ledstrip_materials.add(LedStripSimMaterial {
                effect_data: buffer,
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
        *ledstrip_transform.single_mut() = led_strip_position(e.width, e.height);
        *laser_transform.single_mut() = laser_position(e.width, e.height);
    }
}

fn update_effect(
    time: Res<Time>,
    render_queue: Res<RenderQueue>,
    mut led_strips: ResMut<Assets<LedStripSimMaterial>>,
) {
    let val = time.elapsed().as_nanos().to_le_bytes()[0] as u32;

    println!("val: {:?}", val);

    // for led_strip in &mut led_strips {}
    let mut buffer_data = StorageBuffer::new(Vec::new());
    buffer_data.write(&val).unwrap();
    for (_, material) in led_strips.iter_mut() {
        println!("Sending buffer data: {:?}", buffer_data.as_ref());
        render_queue.write_buffer(&material.effect_data, 4, buffer_data.as_ref());
    }
}
