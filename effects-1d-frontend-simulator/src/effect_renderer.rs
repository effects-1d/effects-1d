use bevy::prelude::*;
use effects_1d_common::{
    color::{self, InterpolatableColor},
    effects::{BlendMode, FrameBufferRef},
};

pub struct SimulationFramebuffer<'a> {
    data: &'a mut [u32],
}

impl<'a> SimulationFramebuffer<'a> {
    pub fn new(data: &'a mut [u32]) -> Self {
        Self { data }
    }
}

impl FrameBufferRef<color::RGB> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::RGB) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = color_to_u32(color)
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::RGB, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let mut existing_color = u32_to_color(*data);
            match blend_mode {
                BlendMode::Add => existing_color += color,
                BlendMode::Max => existing_color.assign_elementwise_max(color),
            }
            *data = color_to_u32(existing_color);
        }
    }
}

impl FrameBufferRef<color::BinaryRGB> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::BinaryRGB) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = color_to_u32(color::RGB {
                r: if color.r { 255 } else { 0 },
                g: if color.g { 255 } else { 0 },
                b: if color.b { 255 } else { 0 },
            });
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::BinaryRGB, _blend_mode: BlendMode) {
        unreachable!();
    }
}

fn mono_to_rgb(color: color::Monochrome) -> color::RGB {
    color::RGB {
        r: color.v,
        g: color.v,
        b: color.v,
    }
}

impl FrameBufferRef<color::Monochrome> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Monochrome) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = color_to_u32(mono_to_rgb(color));
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::Monochrome, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let mut existing_color = u32_to_color(*data);
            let color = mono_to_rgb(color);
            match blend_mode {
                BlendMode::Add => existing_color += color,
                BlendMode::Max => existing_color.assign_elementwise_max(color),
            }
            *data = color_to_u32(existing_color);
        }
    }
}

impl FrameBufferRef<color::Binary> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Binary) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let value = if color.v { 255 } else { 0 };
            *data = color_to_u32(color::RGB {
                r: value,
                g: value,
                b: value,
            });
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::Binary, _blend_mode: BlendMode) {
        unreachable!();
    }
}

/// An object that can render an effect to a simulation framebuffer
#[derive(Resource)]
pub struct EffectRenderer {
    render_callback: Box<dyn FnMut(&mut [u32], &Time) + Send + Sync>,
}

fn color_to_u32(color: color::RGB) -> u32 {
    let r = u32::from(color.r);
    let g = u32::from(color.g);
    let b = u32::from(color.b);

    (r << 0) | (g << 8) | (b << 16)
}

fn u32_to_color(value: u32) -> color::RGB {
    let r = ((value >> 0) & 0xff) as u8;
    let g = ((value >> 8) & 0xff) as u8;
    let b = ((value >> 16) & 0xff) as u8;

    color::RGB { r, g, b }
}

impl EffectRenderer {
    /// Create a new effect renderer
    pub fn new(render_callback: Box<dyn FnMut(&mut [u32], &Time) + Send + Sync>) -> Self {
        Self { render_callback }
    }

    pub(crate) fn render_next_frame(&mut self, framebuffer: &mut [u32], time: &Time) {
        (self.render_callback)(framebuffer, time)
    }
}
