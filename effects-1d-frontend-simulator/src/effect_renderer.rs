use bevy::prelude::*;
use effects_1d_common::{
    color::{
        self,
        palette::{self, FromColor},
        InterpolatableColor,
    },
    effects::{BlendMode, FrameBufferRef},
};

pub struct SimulationFramebuffer<'a> {
    data: &'a mut [color::Oklab],
}

impl<'a> SimulationFramebuffer<'a> {
    pub fn new(data: &'a mut [color::Oklab]) -> Self {
        Self { data }
    }
}

impl FrameBufferRef<color::Oklab> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Oklab) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = color
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::Oklab, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let mut existing_color = *data;
            match blend_mode {
                BlendMode::Add => existing_color += color,
                BlendMode::Max => existing_color.assign_elementwise_max(color),
            }
            *data = existing_color;
        }
    }
}

impl FrameBufferRef<color::BinaryRGB> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::BinaryRGB) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let r = if color.r { 1.0 } else { 0.0 };
            let g = if color.g { 1.0 } else { 0.0 };
            let b = if color.b { 1.0 } else { 0.0 };

            *data = color::Oklab::from_color(palette::Srgb::new(r, g, b));
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::BinaryRGB, _blend_mode: BlendMode) {
        unreachable!();
    }
}

fn mono_to_oklab(color: color::Monochrome) -> color::Oklab {
    let value = f32::from(color.v) / f32::from(u16::MAX);
    color::Oklab::from_color(palette::Srgb::new(value, value, value))
}

impl FrameBufferRef<color::Monochrome> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Monochrome) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = mono_to_oklab(color);
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::Monochrome, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let color = mono_to_oklab(color);
            match blend_mode {
                BlendMode::Add => *data += color,
                BlendMode::Max => data.assign_elementwise_max(color),
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
                mono_to_oklab(color::Monochrome::new(u16::MAX))
            } else {
                mono_to_oklab(color::Monochrome::new(0))
            };
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::Binary, _blend_mode: BlendMode) {
        unreachable!();
    }
}

/// An object that can render an effect to a simulation framebuffer
#[derive(Resource)]
pub struct EffectRenderer {
    render_callback: Box<dyn FnMut(&mut [color::Oklab], &Time) -> String + Send + Sync>,
}

impl EffectRenderer {
    /// Create a new effect renderer
    pub fn new(
        render_callback: Box<dyn FnMut(&mut [color::Oklab], &Time) -> String + Send + Sync>,
    ) -> Self {
        Self { render_callback }
    }

    pub(crate) fn render_next_frame(
        &mut self,
        framebuffer: &mut [color::Oklab],
        time: &Time,
    ) -> String {
        (self.render_callback)(framebuffer, time)
    }
}
