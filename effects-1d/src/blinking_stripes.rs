use effects_1d_common::{
    effects::{BeatBasedEffect, BeatInfo, EffectState},
    errors::RenderError,
};

pub struct BlinkingStripes {
    num_stripes: u16,
}

impl BeatBasedEffect for BlinkingStripes {
    fn render_frame(
        &mut self,
        framebuffer: &mut [u8],
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;

        for i in 0..self.num_stripes {
            if (i.into() + beat.current) % 2 == 0 {
                let i = i as f32;
                let start = i / len;
                let end = (i + 1.0) / len;

                framebuffer.draw_sharp(start, end, green);
            }
        }

        Ok(EffectState {
            over: false,
            idle: true,
        })
    }
}
