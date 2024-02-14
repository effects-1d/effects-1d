use three_d::*;

pub struct LedStripSimMaterial {}

impl LedStripSimMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for LedStripSimMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("ledstrip_sim.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {}
    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            ..Default::default()
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn id(&self) -> u16 {
        0b11u16
    }
}
