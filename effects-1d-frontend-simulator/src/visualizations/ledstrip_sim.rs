use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*,
    sprite::{Material2d, Material2dPlugin},
};

use super::WidgetMaterial;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "69fdd51f-11e6-4ffd-9ebf-2badfdd98f37"]
pub struct LedStripSimMaterial {
    #[storage(0, read_only)]
    pub effect_data: Vec<u32>,

    #[uniform(1)]
    pub widget_size: Vec2,

    pub rel_size: Vec2,
}

impl Material2d for LedStripSimMaterial {
    fn fragment_shader() -> ShaderRef {
        LEDSTRIPSIM_SHADER_HANDLE.typed().into()
    }
}

impl WidgetMaterial for LedStripSimMaterial {
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

pub const LEDSTRIPSIM_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13317158258890179713);

pub struct LedStripSimPlugin;
impl Plugin for LedStripSimPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            LEDSTRIPSIM_SHADER_HANDLE,
            "ledstrip_sim.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(Material2dPlugin::<LedStripSimMaterial>::default());
    }
}
