use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
};

#[derive(Debug)]
pub struct BlinkingStripes {
    num_stripes: u16,
    fix_stripe_size: Option<u32>,
}

impl BeatBasedEffect for BlinkingStripes {
    type Color = color::Binary;

    fn init(resolution_hint: Option<u32>) -> Self {
        let num_stripes = 35;
        let fix_stripe_size = resolution_hint.and_then(|resolution| {
            if resolution < 150 {
                let num_stripes = u32::from(num_stripes);
                Some(((resolution + num_stripes / 2) / num_stripes).max(1))
            } else {
                None
            }
        });

        Self {
            num_stripes,
            fix_stripe_size,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Binary>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        if let Some(fix_stripe_size) = self.fix_stripe_size {
            for i in 0..framebuffer.len() {
                if ((i % (2 * fix_stripe_size)) < fix_stripe_size) ^ (beat.current % 2 == 0) {
                    framebuffer.set_pixel(i, color::Binary::on());
                }
            }
        } else {
            let len = self.num_stripes as f32;
            for i in 0..self.num_stripes {
                let i: i32 = i.into();
                if (i + beat.current / 2) % 2 == 0 {
                    let i = i as f32;
                    let start = i / len;
                    let end = (i + 1.0) / len;

                    framebuffer.draw_sharp(start, end, color::Binary::on());
                }
            }
        }

        Ok(EffectState { idle: true })
    }
}
