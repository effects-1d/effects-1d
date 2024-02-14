use three_d::*;

pub struct SimMaterial {
    context: Context,
    data: Texture2D,
    data_size: u32,
    fragment_shader: &'static str,
}

fn create_1d_texture(context: &Context, len: u32) -> Texture2D {
    Texture2D::new_empty::<[f32; 3]>(
        context,
        len,
        1,
        Interpolation::Nearest,
        Interpolation::Nearest,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    )
}

impl SimMaterial {
    fn new(context: &Context, fragment_shader: &'static str) -> Self {
        Self {
            context: context.clone(),
            data: create_1d_texture(context, 1),
            data_size: 1,
            fragment_shader,
        }
    }

    pub fn ledstrip(context: &Context) -> Self {
        Self::new(context, include_str!("ledstrip_sim.frag"))
    }

    pub fn update_data(&mut self, data: &[[f32; 3]]) {
        self.data_size = data.len() as u32;
        if self.data_size != self.data.width() {
            self.data = create_1d_texture(&self.context, self.data_size);
        }
        self.data.fill(data);
    }
}

impl Material for SimMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        self.fragment_shader.to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_texture("data", &self.data);
    }

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
