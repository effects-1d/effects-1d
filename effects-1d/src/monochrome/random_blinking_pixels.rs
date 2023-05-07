use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
    random::{rand::seq::SliceRandom, EffectRng},
    rhythm::MultiBeatCycle,
};

// Parameters
const NUM_LINES: usize = 50;
const CYCLE_LENGTH: u16 = 5;

#[derive(Debug)]
pub struct RandomBlinkingPixels {
    offsets: [f32; NUM_LINES],
    cycle: MultiBeatCycle,
}

impl BeatBasedEffect for RandomBlinkingPixels {
    type Color = color::Monochrome;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut rng = EffectRng::new();
        let mut offsets = core::array::from_fn(|pos| (pos as f32) / (NUM_LINES as f32));
        offsets.shuffle(&mut rng);
        Self {
            offsets,
            cycle: MultiBeatCycle::new(CYCLE_LENGTH, start_beat),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        let progress = self.cycle.cycle_progress();
        let line_width = 1.0 / (self.offsets.len() as f32);
        for (pos, offset) in self.offsets.iter().enumerate() {
            let start = (pos as f32) / (self.offsets.len() as f32);
            let end = start + line_width;

            let intensity = 1.0 - (progress + offset).rem_euclid(1.0);
            let intensity = intensity * intensity;
            framebuffer.draw_sharp(start, end, intensity.into());
        }

        Ok(EffectState {
            idle: self.cycle.cycle_number() >= 2,
        })
    }
}
