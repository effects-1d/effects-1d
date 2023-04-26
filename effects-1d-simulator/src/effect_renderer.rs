use bevy::prelude::*;
use effects_1d_common::{
    color::{self, BlendableColor},
    effects::{BlendMode, FrameBufferRef},
};

pub struct SimulationFramebuffer<'a> {
    data: &'a mut [color::RGB],
}

impl<'a> SimulationFramebuffer<'a> {
    pub fn new(data: &'a mut [color::RGB]) -> Self {
        Self { data }
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

    fn update_pixel_with_transparent_color(
        &mut self,
        pos: u32,
        color: color::TransparentColor<color::RGB>,
    ) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = data.elementwise_lerp(color.value, color.alpha);
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

    fn update_pixel(&mut self, _pos: u32, _color: color::BinaryRGB, _blend_mode: BlendMode) {
        unreachable!();
    }

    fn update_pixel_with_transparent_color(
        &mut self,
        _pos: u32,
        _color: color::TransparentColor<color::BinaryRGB>,
    ) {
        unreachable!();
    }
}

fn mono_to_rgb(color: color::Monochrome) -> color::RGB {
    let value = color.v;
    color::RGB::new(value, value, value)
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

    fn update_pixel_with_transparent_color(
        &mut self,
        pos: u32,
        color: color::TransparentColor<color::Monochrome>,
    ) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let color_value = mono_to_rgb(color.value);
            *data = data.elementwise_lerp(color_value, color.alpha);
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

    fn update_pixel(&mut self, _pos: u32, _color: color::Binary, _blend_mode: BlendMode) {
        unreachable!();
    }

    fn update_pixel_with_transparent_color(
        &mut self,
        _pos: u32,
        _color: color::TransparentColor<color::Binary>,
    ) {
        unreachable!();
    }
}

/// An object that can render an effect to a simulation framebuffer
#[derive(Resource)]
pub struct EffectRenderer {
    render_callback: Box<dyn FnMut(&mut [color::RGB], &Time) -> String + Send + Sync>,
}

impl EffectRenderer {
    /// Create a new effect renderer
    pub fn new(
        render_callback: Box<dyn FnMut(&mut [color::RGB], &Time) -> String + Send + Sync>,
    ) -> Self {
        Self { render_callback }
    }

    pub(crate) fn render_next_frame(
        &mut self,
        framebuffer: &mut [color::RGB],
        time: &Time,
    ) -> String {
        (self.render_callback)(framebuffer, time)
    }
}
