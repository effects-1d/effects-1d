#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    rhythm::Sequencer,
};

#[derive(Debug)]
pub struct StrobingRotatingLines {
    num_lines: u16,
    line_size: f32,
    rotation_speed: f32,
    rotation_position: f32,
    rotation_delay_beats: u8,
    line_id_offset: u16,
    sequencer: Sequencer,
}

impl ConstructibleBeatBasedEffect for StrobingRotatingLines {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut this = Self {
            num_lines: 28,
            line_size: 0.005,
            rotation_speed: 0.02,
            rotation_position: 0.0,
            line_id_offset: 0,
            rotation_delay_beats: 8,
            sequencer: Sequencer::new(1, 4, &[0b0011, 0b1100], 0b1000, 8, start_beat).into(),
        };

        if let Some(resolution) = resolution_hint {
            let pixel_size = 1.0 / (resolution as f32);
            if this.line_size < pixel_size {
                this.line_size = pixel_size;
            }
        }

        this
    }
}

impl BeatBasedEffect for StrobingRotatingLines {
    type Color = color::Binary;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Binary>,
        d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        // Update state
        self.sequencer.update(beat);

        // Line positions
        let space_between =
            (1.0 - self.line_size * f32::from(self.num_lines)) / f32::from(self.num_lines + 1);
        let line_stride = space_between + self.line_size;
        let line_offset = space_between;

        // Rotation position
        if self.sequencer.beat().current >= i32::from(self.rotation_delay_beats) {
            self.rotation_position += self.rotation_speed * d_t;
            if self.rotation_position > line_stride {
                self.rotation_position -= line_stride;
                self.line_id_offset += 1;
                if self.line_id_offset >= 4 {
                    self.line_id_offset -= 4;
                }
            }
        }

        // Draw
        for line_id in 0..=self.num_lines {
            let sequence_id = ((line_id + self.line_id_offset) / 2) % 2;

            let duty_position = self.sequencer.beat().fractional;
            if duty_position > 0.2 && self.sequencer.sequence_value(sequence_id) {
                let start = (line_offset + line_stride * f32::from(line_id)
                    - self.rotation_position)
                    * 1.00001; // Prevent clipping at the end
                let end = start + self.line_size;

                framebuffer.draw_sharp(start, end, color::Binary::on());
            }
        }

        Ok(EffectState {
            idle: self.sequencer.ready_for_unscheduling(),
        })
    }
}
