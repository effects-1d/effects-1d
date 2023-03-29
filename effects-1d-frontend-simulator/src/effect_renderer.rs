use bevy::prelude::*;

// TODO move rendering in separate struct so it can keep its state
#[derive(Resource)]
pub struct EffectRenderer {}

impl EffectRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_next_frame(&mut self, framebuffer: &mut [u32], time: &Time) {
        let val = time.elapsed().as_nanos().to_le_bytes()[3] as u32;
        if let Some(fb_val) = framebuffer.get_mut(framebuffer.len() / 2) {
            *fb_val = val;
        };
    }
}
