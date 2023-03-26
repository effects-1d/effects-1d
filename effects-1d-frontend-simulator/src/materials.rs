use bevy::{reflect::TypeUuid, render::render_resource::*, sprite::Material2d};

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
pub struct LedStripSimMaterial {}

impl Material2d for LedStripSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/led_strip.wgsl".into()
    }
}
