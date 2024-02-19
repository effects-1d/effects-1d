use three_d::*;

pub struct SimMaterial {
    context: Context,
    data: Texture2D,
    data_size: u32,
    fragment_shader: &'static str,
    widget_size: Vec2,
    time: f32,
    material_id: u16,
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
    fn new(context: &Context, fragment_shader: &'static str, material_id: u16) -> Self {
        Self {
            context: context.clone(),
            data: create_1d_texture(context, 1),
            data_size: 1,
            fragment_shader,
            widget_size: Vec2::new(1.0, 1.0),
            time: 0.0,
            material_id,
        }
    }

    pub fn ledstrip(context: &Context) -> Self {
        Self::new(context, include_str!("shaders/sim_ledstrip.frag"), 0b101u16)
    }
    pub fn laser(context: &Context) -> Self {
        Self::new(context, include_str!("shaders/sim_laser.frag"), 0b100u16)
    }

    pub fn update_data(&mut self, data: &[[f32; 3]]) {
        self.data_size = data.len() as u32;
        if self.data_size != self.data.width() {
            self.data = create_1d_texture(&self.context, self.data_size);
        }
        self.data.fill(data);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.widget_size.x = width;
        self.widget_size.y = height;
    }

    pub fn set_time(&mut self, time: f32) {
        self.time = time;
    }
}

impl Material for SimMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        format!(
            "{}\n{}",
            include_str!("shaders/sim_common.frag"),
            self.fragment_shader
        )
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_texture("effect_data", &self.data);
        program.use_uniform_if_required("effect_data_len", &self.data_size);
        program.use_uniform_if_required("widget_size", &self.widget_size);
        program.use_uniform_if_required("time", &self.time);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }

    fn id(&self) -> u16 {
        self.material_id
    }
}
