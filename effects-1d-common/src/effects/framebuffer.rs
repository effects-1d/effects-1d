use crate::color::{gradients::ColorGradient, BlendableColor, Color, TransparentColor};

use super::BlendMode;

/// A reference to a framebuffer an effect can render into.
pub trait FrameBufferRef<C: Color> {
    /// Get the resolution of the framebuffer.
    fn len(&self) -> u32;

    /// Set a specific pixel in the framebuffer.
    ///
    /// Replaces the previous color of the pixel.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the pixel that should get modified.
    ///           Should be in the range of `0` to `len() - 1`.
    ///           Can be outside of this range, but then nothing will happen.
    /// * `color` - The color the pixel shall be set to.
    fn set_pixel(&mut self, pos: u32, color: C);

    /// Updates a specific pixel in the framebuffer.
    ///
    /// Update the existing color of the pixel based on the given blend mode.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the pixel that should get modified.
    ///           Should be in the range of `0` to `len() - 1`.
    ///           Can be outside of this range, but then nothing will happen.
    /// * `color` - The color the pixel shall be updated with.
    /// * `blend_mode` - The mechanism that should be used to combine the existing
    ///                  and the new pixel color
    fn update_pixel(&mut self, pos: u32, color: C, blend_mode: BlendMode)
    where
        C: BlendableColor;

    /// Updates a specific pixel in the framebuffer.
    ///
    /// Update the existing color of the pixel based on transparency blending.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the pixel that should get modified.
    ///           Should be in the range of `0` to `len() - 1`.
    ///           Can be outside of this range, but then nothing will happen.
    /// * `color` - The color and transparency the pixel shall be updated with.
    fn update_pixel_with_transparent_color(&mut self, pos: u32, color: TransparentColor<C>)
    where
        C: BlendableColor;

    /// Draw a segment with sharp edges, replacing the existing color within the segment.
    ///
    /// This function performs no anti-aliasing, so all the pixels that
    /// get modified are exactly the given color.
    ///
    /// Multiple sections that touch each other will be rendered seamless.
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

    /// Draw a segment with sharp edges, replacing the existing color within the segment.
    ///
    /// This function performs no anti-aliasing, so all the pixels that
    /// get modified are exactly the given color.
    ///
    /// Multiple sections that touch each other will be rendered seamless.
    ///
    /// # Arguments
    ///
    /// * `start`, `end` - The range that should be filled.
    ///                    The left end of the framebuffer is `0.0`, the right end is `1.0`.
    /// * `color` - The color the range shall be set to.
    fn draw_smooth(&mut self, start: f32, end: f32, color: C, blend_mode: BlendMode)
    where
        C: BlendableColor,
    {
        let len = self.len();
        let len_f = len as f32;
        let start = len_f * start;
        let end = len_f * end;

        // Don't draw negative ranges
        if end < start {
            return;
        }

        // Don't draw ranges that are fully out of bounds
        if end < 0.0 || start > len_f {
            return;
        }

        // Clamp to valid range
        let start = start.clamp(0.0, len_f);
        let end = end.clamp(0.0, len_f);

        let start_pixel = (start as u32).clamp(0, len - 1);
        let end_pixel = (end as u32).clamp(0, len - 1);

        // If we hit only one pixel, draw that one pixel
        if start_pixel == end_pixel {
            let color = color.multiply_with(end - start);
            self.update_pixel(start_pixel, color, blend_mode);
            return;
        }

        // Else, draw range as normal.
        // Start with the first/last pixel, as those are special cases
        {
            // How much the area reaches into the first pixel
            let amount = (start_pixel + 1) as f32 - start;
            let color = color.multiply_with(amount);
            self.update_pixel(start_pixel, color, blend_mode);
        }
        {
            // How much the area reaches into the last pixel
            let amount = end - end_pixel as f32;
            let color = color.multiply_with(amount);
            self.update_pixel(end_pixel, color, blend_mode);
        }

        for pos in (start_pixel + 1)..end_pixel {
            self.update_pixel(pos, color, blend_mode);
        }
    }

    /// Draw a gradient
    fn draw_gradient(&mut self, start: f32, end: f32, gradient: &dyn ColorGradient<C = C>) {
        let len = self.len();
        let len_f = len as f32;
        let start = len_f * start;
        let end = len_f * end;

        // Don't draw negative ranges
        if end < start {
            return;
        }

        // Don't draw ranges that are fully out of bounds
        if end < 0.5 || start > (len_f - 0.5) {
            return;
        }

        // Draw range
        let start_pixel = (start + 0.5) as u32;
        let end_pixel = (end - 0.5).max(0.0) as u32;
        for pos in start_pixel..=end_pixel {
            let posf = ((pos as f32 + 0.5) - start) / (end - start);
            self.set_pixel(pos, gradient.interpolate(posf));
        }
    }

    /// Draw a gradient
    fn draw_gradient_with_blend(
        &mut self,
        start: f32,
        end: f32,
        gradient: &dyn ColorGradient<C = C>,
        blend_mode: BlendMode,
    ) where
        C: BlendableColor,
    {
        let len = self.len();
        let len_f = len as f32;
        let start = len_f * start;
        let end = len_f * end;

        // Don't draw negative ranges
        if end < start {
            return;
        }

        // Don't draw ranges that are fully out of bounds
        if end < 0.5 || start > (len_f - 0.5) {
            return;
        }

        // Draw range
        let start_pixel = (start + 0.5) as u32;
        let end_pixel = (end - 0.5).max(0.0) as u32;
        for pos in start_pixel..=end_pixel {
            let posf = ((pos as f32 + 0.5) - start) / (end - start);
            self.update_pixel(pos, gradient.interpolate(posf), blend_mode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::color::Monochrome;

    const ON: color::Binary = color::Binary::on();
    const OFF: color::Binary = color::Binary::off();

    /// A framebuffer an effect can render into.
    pub struct TestFrameBuffer<'a, C> {
        data: &'a mut [C],
    }

    impl<'a, C: Color> FrameBufferRef<C> for TestFrameBuffer<'_, C> {
        fn len(&self) -> u32 {
            self.data.len() as u32
        }

        fn set_pixel(&mut self, pos: u32, color: C) {
            if let Some(v) = self.data.get_mut(pos as usize) {
                *v = color;
            }
        }

        fn update_pixel(&mut self, pos: u32, color: C, blend_mode: BlendMode)
        where
            C: BlendableColor,
        {
            if let Some(v) = self.data.get_mut(pos as usize) {
                match blend_mode {
                    BlendMode::Add => v.elementwise_add(color),
                    BlendMode::Max => v.assign_elementwise_max(color),
                };
            }
        }

        fn update_pixel_with_transparent_color(&mut self, pos: u32, color: TransparentColor<C>)
        where
            C: BlendableColor,
        {
            if let Some(v) = self.data.get_mut(pos as usize) {
                *v = v.multiply_with(1. - color.alpha);
                v.elementwise_add(color.value.multiply_with(color.alpha));
            }
        }
    }

    #[test]
    fn draw_sharp_singlepixel_round_down() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.38, 0.41, ON);

        assert_eq!(data, [OFF, ON, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_singlepixel_round_up() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.39, 0.42, ON);

        assert_eq!(data, [OFF, OFF, ON, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_multipixel_exclusive() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.31, 0.69, ON);

        assert_eq!(data, [OFF, OFF, ON, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_multipixel_inclusive() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.29, 0.71, ON);

        assert_eq!(data, [OFF, ON, ON, ON, OFF]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_low() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(-10.0, 0.01, ON);

        assert_eq!(data, [ON, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_high() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.99, 10.0, ON);

        assert_eq!(data, [OFF, OFF, OFF, OFF, ON]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_full() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(-10.0, -9.8, ON);
        framebuffer.draw_sharp(-10.0, -5.0, ON);
        framebuffer.draw_sharp(5.0, 10.0, ON);
        framebuffer.draw_sharp(9.8, 10.0, ON);

        assert_eq!(data, [OFF, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_sharp_out_of_bounds_both_sides() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(-10.0, 10.0, ON);

        assert_eq!(data, [ON, ON, ON, ON, ON]);
    }

    #[test]
    fn draw_sharp_negative_range() {
        let mut data = [OFF; 5];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_sharp(0.9, 0.1, ON);

        assert_eq!(data, [OFF, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn draw_smooth_normal_range() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(0.13, 0.58, Monochrome::new(100), BlendMode::Add);

        assert_eq!(data, [0, 70, 100, 100, 100, 80, 0, 0, 0, 0]);
    }

    #[test]
    fn draw_smooth_single_pixel() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(0.13, 0.175, Monochrome::new(100), BlendMode::Add);

        assert_eq!(data, [0, 45, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn draw_smooth_out_of_bounds_full() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(-10.0, -9.8, Monochrome::new(100), BlendMode::Add);
        framebuffer.draw_smooth(-10.0, -5.0, Monochrome::new(100), BlendMode::Add);
        framebuffer.draw_smooth(5.0, 10.0, Monochrome::new(100), BlendMode::Add);
        framebuffer.draw_smooth(9.8, 10.0, Monochrome::new(100), BlendMode::Add);

        assert_eq!(data, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn draw_smooth_out_of_bounds_both_sides() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(-10.0, 10.0, Monochrome::new(100), BlendMode::Add);

        assert_eq!(data, [100, 100, 100, 100, 100, 100, 100, 100, 100, 100]);
    }

    #[test]
    fn draw_smooth_overlap_add() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(0.11, 0.65, Monochrome::new(100), BlendMode::Add);
        framebuffer.draw_smooth(0.45, 0.85, Monochrome::new(50), BlendMode::Add);

        assert_eq!(data, [0, 90, 100, 100, 125, 150, 100, 50, 25, 0]);
    }

    #[test]
    fn draw_smooth_overlap_max() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(0.11, 0.65, Monochrome::new(100), BlendMode::Max);
        framebuffer.draw_smooth(0.45, 0.85, Monochrome::new(50), BlendMode::Max);

        assert_eq!(data, [0, 90, 100, 100, 100, 100, 50, 50, 25, 0]);
    }

    #[test]
    fn draw_smooth_touch_add() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(0.1, 0.52, Monochrome::new(100), BlendMode::Add);
        framebuffer.draw_smooth(0.52, 0.9, Monochrome::new(100), BlendMode::Add);

        assert_eq!(data, [0, 100, 100, 100, 100, 100, 100, 100, 100, 0]);
    }

    #[test]
    fn draw_smooth_touch_max() {
        let mut data = [Monochrome::zero(); 10];
        let mut framebuffer = TestFrameBuffer { data: &mut data };

        framebuffer.draw_smooth(0.1, 0.52, Monochrome::new(100), BlendMode::Max);
        framebuffer.draw_smooth(0.52, 0.9, Monochrome::new(100), BlendMode::Max);

        assert_eq!(data, [0, 100, 100, 100, 100, 80, 100, 100, 100, 0]);
    }
}
