use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
};

pub struct BlinkingStripes {
    num_stripes: u16,
}

impl BeatBasedEffect for BlinkingStripes {
    type Color = color::Binary;

    fn init() -> Self {
        todo!()
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;

        for i in 0..self.num_stripes {
            let i: i32 = i.into();
            if (i + beat.current) % 2 == 0 {
                let i = i as f32;
                let start = i / len;
                let end = (i + 1.0) / len;

                framebuffer.draw_sharp(start, end, color::Binary::on());
            }
        }

        Ok(EffectState {
            over: false,
            idle: true,
        })
    }
}
