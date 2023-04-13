use bevy::prelude::*;
use effects_1d_common::{
    color::{self, InterpolatableColor, Okhsv},
    effects::{BlendMode, FrameBufferRef},
};

pub struct SimulationFramebuffer<'a> {
    data: &'a mut [color::Okhsv],
}

impl<'a> SimulationFramebuffer<'a> {
    pub fn new(data: &'a mut [color::Okhsv]) -> Self {
        Self { data }
    }
}

impl FrameBufferRef<color::Okhsv> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Okhsv) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = color
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::Okhsv, blend_mode: BlendMode) {
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
            *data = match (color.r, color.g, color.b) {
                (true, true, true) => Okhsv::new(0., 0., 1.),
                (true, false, false) => Okhsv::new(29., 1., 1.),
                (true, true, false) => Okhsv::new(110., 1., 1.),
                (false, true, false) => Okhsv::new(142., 1., 1.),
                (false, true, true) => Okhsv::new(195., 1., 1.),
                (false, false, true) => Okhsv::new(264., 1., 1.),
                (true, false, true) => Okhsv::new(328., 1., 1.),
                (false, false, false) => Okhsv::new(0., 0., 0.),
            };
        }
    }

    fn update_pixel(&mut self, _pos: u32, _color: color::BinaryRGB, _blend_mode: BlendMode) {
        unreachable!();
    }
}

fn mono_to_okhsv(color: color::Monochrome) -> color::Okhsv {
    Okhsv::new(0., 0., f32::from(color.v) / 65535.0)
}

impl FrameBufferRef<color::Monochrome> for SimulationFramebuffer<'_> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: color::Monochrome) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            *data = mono_to_okhsv(color);
        }
    }

    fn update_pixel(&mut self, pos: u32, color: color::Monochrome, blend_mode: BlendMode) {
        if let Some(data) = self.data.get_mut(pos as usize) {
            let color = mono_to_okhsv(color);
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
                Okhsv::new(0., 0., 1.)
            } else {
                Okhsv::new(0., 0., 0.)
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
    render_callback: Box<dyn FnMut(&mut [color::Okhsv], &Time) -> String + Send + Sync>,
}

impl EffectRenderer {
    /// Create a new effect renderer
    pub fn new(
        render_callback: Box<dyn FnMut(&mut [color::Okhsv], &Time) -> String + Send + Sync>,
    ) -> Self {
        Self { render_callback }
    }

    pub(crate) fn render_next_frame(
        &mut self,
        framebuffer: &mut [color::Okhsv],
        time: &Time,
    ) -> String {
        (self.render_callback)(framebuffer, time)
    }
}
