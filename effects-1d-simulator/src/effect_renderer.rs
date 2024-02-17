use effects_1d_common::{
    color::{self, BlendableColor, Color, RGB},
    effects::{BeatInfo, BlendMode, FrameBufferRef},
};

pub struct SimulationFramebuffer<'a> {
    data: &'a mut [color::RGB],
}

impl<'a> SimulationFramebuffer<'a> {
    pub fn new(data: &'a mut [color::RGB]) -> Self {
        Self { data }
    }

    pub fn fade(&mut self, progress: f32) {
        for pixel in self.data.iter_mut() {
            *pixel = pixel.elementwise_lerp(RGB::zero(), progress);
        }
    }
}

impl FrameBufferRef<color::RGB> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::RGB) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = color
        }
    }

    fn set_pixels(&mut self, pixel_fn: &dyn Fn(u32) -> color::RGB) {
        for (pos, v) in self.data.iter_mut().enumerate() {
            *v = pixel_fn(pos as u32);
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::RGB, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = match blend_mode {
                BlendMode::Add => data.elementwise_add(color),
                BlendMode::Max => data.elementwise_max(color),
                BlendMode::None => color,
                BlendMode::Alpha(alpha) => data.elementwise_lerp(color, alpha),
            };
        }
    }
}

impl FrameBufferRef<color::BinaryRGB> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::BinaryRGB) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let r = if color.r { u16::MAX } else { 0 };
            let g = if color.g { u16::MAX } else { 0 };
            let b = if color.b { u16::MAX } else { 0 };

            *data = color::RGB::new(r, g, b);
        }
    }

    fn set_pixels(&mut self, pixel_fn: &dyn Fn(u32) -> color::BinaryRGB) {
        for (pos, v) in self.data.iter_mut().enumerate() {
            let color = pixel_fn(pos as u32);

            let r = if color.r { u16::MAX } else { 0 };
            let g = if color.g { u16::MAX } else { 0 };
            let b = if color.b { u16::MAX } else { 0 };

            *v = color::RGB::new(r, g, b);
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::BinaryRGB, _blend_mode: BlendMode) {
        unreachable!();
    }
}

fn mono_to_rgb(color: color::Monochrome) -> color::RGB {
    let value = color.v;
    color::RGB::new(0, value, 0)
}

impl FrameBufferRef<color::Monochrome> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Monochrome) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = mono_to_rgb(color);
        }
    }

    fn set_pixels(&mut self, pixel_fn: &dyn Fn(u32) -> color::Monochrome) {
        for (pos, v) in self.data.iter_mut().enumerate() {
            *v = mono_to_rgb(pixel_fn(pos as u32));
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::Monochrome, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let color = mono_to_rgb(color);
            *data = match blend_mode {
                BlendMode::Add => data.elementwise_add(color),
                BlendMode::Max => data.elementwise_max(color),
                BlendMode::None => color,
                BlendMode::Alpha(alpha) => data.elementwise_lerp(color, alpha),
            }
        }
    }
}

impl FrameBufferRef<color::Binary> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Binary) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = if color.v {
                mono_to_rgb(color::Monochrome::new(u16::MAX))
            } else {
                mono_to_rgb(color::Monochrome::new(0))
            };
        }
    }

    fn set_pixels(&mut self, pixel_fn: &dyn Fn(u32) -> color::Binary) {
        for (pos, v) in self.data.iter_mut().enumerate() {
            *v = if pixel_fn(pos as u32).v {
                mono_to_rgb(color::Monochrome::new(u16::MAX))
            } else {
                mono_to_rgb(color::Monochrome::new(0))
            };
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::Binary, _blend_mode: BlendMode) {
        unreachable!();
    }
}

/// An object that can render an effect to a simulation framebuffer
pub struct EffectRenderer {
    render_callback: Box<dyn FnMut(&mut [color::RGB], f32, BeatInfo) -> String + Send + Sync>,
}

impl EffectRenderer {
    /// Create a new effect renderer
    pub fn new(
        render_callback: Box<dyn FnMut(&mut [color::RGB], f32, BeatInfo) -> String + Send + Sync>,
    ) -> Self {
        Self { render_callback }
    }

    pub(crate) fn render_next_frame(
        &mut self,
        framebuffer: &mut [color::RGB],
        time: f32,
        beat: BeatInfo,
    ) -> String {
        (self.render_callback)(framebuffer, time, beat)
    }
}
