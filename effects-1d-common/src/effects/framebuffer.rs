use crate::color::Color;

/// A reference to a framebuffer an effect can render into.
pub trait FrameBufferRef<C: Color> {
    /// Get the resolution of the framebuffer.
    fn len(&self) -> u32;

    /// Set a specific pixel in the framebuffer.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the pixel that should get modified.
    ///           Should be in the range of `0` to `len() - 1`.
    ///           Can be outside of this range, but then nothing will happen.
    /// * `color` - The color the pixel shall be set to.
    fn set_pixel(&mut self, pos: u32, color: C);

    /// Draw a segment with sharp edges.
    ///
    /// This function performs no anti-aliasing, so all the pixels that
    /// get modified are exactly the given color.
    ///
    /// Rendering multiple sections that touch each other will not cause any
    /// seams in between.
    ///
    /// # Arguments
    ///
    /// * `start`, `end` - The range that should be filled.
    ///                    The left end of the framebuffer is `0.0`, the right end is `1.0`.
    /// * `color` - The color the range shall be set to.
    fn draw_sharp(&mut self, start: f32, end: f32, color: C) {
        let len = self.len() as f32;
        let start = len * start;
        let end = len * end;

        // Don't draw negative ranges
        if end < start {
            return;
        }

        // Don't draw ranges that are fully out of bounds
        if end < 0.0 || start > len {
            return;
        }

        // Clamp to valid range
        let start = start.clamp(0.0, len);
        let end = end.clamp(0.0, len);

        // If range is very short, draw exactly one pixel, even if we technically
        // didn't hit any pixel centers.
        if end - start <= 1.0 {
            let mean = (end + start) / 2.0;
            let pos = (mean as u32).clamp(0, self.len() - 1);
            self.set_pixel(pos, color);
            return;
        }

        // Else, draw range as normal
        let start = (start + 0.5) as u32;
        let end = (end - 0.5).max(0.0) as u32;
        for pos in start..=end {
            self.set_pixel(pos, color);
        }
    }
}

/// A framebuffer an effect can render into.
pub struct FrameBuffer<'a, C> {
    data: &'a mut [C],
}

impl<'a, C: Color> FrameBufferRef<C> for FrameBuffer<'_, C> {
    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn set_pixel(&mut self, pos: u32, color: C) {
        if let Some(v) = self.data.get_mut(pos as usize) {
            *v = color;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    const ON: color::Binary = color::Binary::on();
    const OFF: color::Binary = color::Binary::off();

    #[test]
    fn draw_sharp_singlepixel_round_down() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.38, 0.41, ON);

        assert_eq!(data, [OFF, ON, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_singlepixel_round_up() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.39, 0.42, ON);

        assert_eq!(data, [OFF, OFF, ON, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_multipixel_exclusive() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.31, 0.69, ON);

        assert_eq!(data, [OFF, OFF, ON, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_multipixel_inclusive() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.29, 0.71, ON);

        assert_eq!(data, [OFF, ON, ON, ON, OFF]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_low() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(-10.0, 0.01, ON);

        assert_eq!(data, [ON, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_high() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.99, 10.0, ON);

        assert_eq!(data, [OFF, OFF, OFF, OFF, ON]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_full() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(-10.0, -9.8, ON);
        framebuffer.draw_sharp(-10.0, -5.0, ON);
        framebuffer.draw_sharp(5.0, 10.0, ON);
        framebuffer.draw_sharp(9.8, 10.0, ON);

        assert_eq!(data, [OFF, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_both_sides() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(-10.0, 10.0, ON);

        assert_eq!(data, [ON, ON, ON, ON, ON]);
    }

    #[test]
    fn draw_sharp_negative_range() {
        let mut data = [OFF; 5];
        let mut framebuffer = FrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.9, 0.1, ON);

        assert_eq!(data, [OFF, OFF, OFF, OFF, OFF]);
    }
}
