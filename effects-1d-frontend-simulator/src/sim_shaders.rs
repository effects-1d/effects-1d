use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*, sprite::Material2d};

#[derive(Component)]
pub struct LaserSim;

#[derive(Component)]
pub struct LedStripSim;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a71be379-cac5-4204-8dbd-33982a9e1a60"]
pub struct LaserSimMaterial {}

impl Material2d for LaserSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/laser.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "69fdd51f-11e6-4ffd-9ebf-2badfdd98f37"]
pub struct LedStripSimMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material2d for LedStripSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/led_strip.wgsl".into()
    }
}

pub fn led_strip_position(window_width: f32, window_height: f32) -> Transform {
    Transform::default()
        .with_scale(Vec3::new(window_width, window_height * 0.05, 0.))
        .with_translation(Vec3::new(0., window_height * (0.5 - 0.05), 0.))
}

pub fn laser_position(window_width: f32, window_height: f32) -> Transform {
    Transform::default()
        .with_scale(Vec3::new(window_width, window_height * 0.9, 0.))
        .with_translation(Vec3::new(0., -window_height * 0.05, 0.))
}
