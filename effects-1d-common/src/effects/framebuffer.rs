/// A reference to a framebuffer an effect can render into.
pub trait FrameBufferRef<'a, C> {
    /// Returns the resolution of the framebuffer
    fn len(&self) -> u32;
    /// Sets a specific pixel in the framebuffer
    fn set_pixel(&mut self, pos: u32, color: C);
    /// Draws a segment with sharp edges
    fn draw_sharp(&mut self, start: f32, end: f32, color: C) {
        let len = self.len() as f32;
        let start = (len * start + 0.5) as i32;
        let end = (len * end - 0.5) as i32;

        //for pos in start .. end
    }
}

/// A framebuffer an effect can render into.
pub struct FrameBuffer<'a, C> {
    data: &'a mut [C],
}

impl<'a, C> FrameBufferRef<'a, C> for &mut FrameBuffer<'_, C> {
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
