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
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_scale(Vec3::new(512., 256., 0.))
            .with_translation(Vec3::new(-100., 0., 0.)),
        material: materials.add(CustomMaterial {}),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
