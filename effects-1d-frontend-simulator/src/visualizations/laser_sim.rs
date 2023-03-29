use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*, sprite::Material2d};

use super::WidgetMaterial;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a71be379-cac5-4204-8dbd-33982a9e1a60"]
pub struct LaserSimMaterial {
    #[storage(0, read_only)]
    pub effect_data: Vec<u32>,

    #[uniform(1)]
    pub widget_size: Vec2,

    pub rel_size: Vec2,
}

impl Material2d for LaserSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/laser.wgsl".into()
    }
}

impl WidgetMaterial for LaserSimMaterial {
    fn new(rel_size: Vec2, window_size: Vec2) -> Self {
        let mut this = Self {
            effect_data: vec![0],
            widget_size: default(),
            rel_size,
        };
        this.update_window_size(window_size);
        this
    }

    fn update_window_size(&mut self, window_size: Vec2) {
        self.widget_size = self.rel_size * window_size;
    }
}
