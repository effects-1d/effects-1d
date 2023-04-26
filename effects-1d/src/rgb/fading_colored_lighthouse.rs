use effects_1d_common::{
    color::{self, BlendableColor, RGB},
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    random::{EffectRng, Rng},
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct FadingColoredLighthouse {
    line_width: f32,
    cycle: MultiBeatCycle,
    rng: EffectRng,
    pos_start: f32,
    pos_end: f32,
    // left_to_right: bool,
    color: color::RGB,
}

impl FadingColoredLighthouse {
    fn regenerate(&mut self) {
        let range = 1.0 - self.line_width;

        let pos1 = range * self.rng.gen::<f32>();
        let pos2 = range * self.rng.gen::<f32>();

        // if self.left_to_right == (pos1 > pos2) {
        //     self.pos_start = pos1;
        //     self.pos_end = pos2;
        // } else {
        //     self.pos_start = pos2;
        //     self.pos_end = pos1;
        // }

        self.pos_start = pos1;
        self.pos_end = pos2;

        // self.left_to_right = !self.left_to_right;

        self.color = color::hsv8(self.rng.gen::<f32>() * 360.0, 1.0, 1.0);
    }
}

impl BeatBasedEffect for FadingColoredLighthouse {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut this = Self {
            line_width: 0.1,
            cycle: MultiBeatCycle::new(4, start_beat),
            rng: EffectRng::new(),
            pos_start: 0.0,
            pos_end: 0.0,
            // left_to_right: true,
            color: RGB::new(0, 0, 0),
        };

        this.regenerate();

        this
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        if self.cycle.is_new_cycle() {
            self.regenerate();
        }

        let progress = self.cycle.cycle_progress();

        let pos = self.cycle.lerp(self.pos_start, self.pos_end);

        let brightness = if progress < 0.5 {
            2.0 * progress
        } else {
            2.0 * (1.0 - progress)
        };

        let color = self.color.multiply_with(brightness);

        framebuffer.draw_smooth(pos, pos + self.line_width, color, BlendMode::Add);

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
