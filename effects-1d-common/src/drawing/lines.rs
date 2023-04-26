/// Utility for rendering evenly spaced lines on the entire frame
#[derive(Debug)]
pub struct Lines {
    line_width: f32,
    line_stride: f32,
    num_lines: u16,
    cycle_offset: f32,
}

impl Lines {
    /// Create a new lines utility object
    pub fn new(line_width: f32, line_stride: f32, cycle_offset: f32) -> Self {
        assert!(cycle_offset >= 0.0 && cycle_offset <= 1.0);
        let num_lines = (1.0 / line_stride) as u16 + 2;
        Self {
            line_width,
            line_stride,
            cycle_offset,
            num_lines,
        }
    }

    /// Creates a new lines utility object where the single lines are at least one pixel wide
    pub fn new_with_resolution_hint(
        mut line_width: f32,
        mut line_stride: f32,
        cycle_offset: f32,
        resolution_hint: Option<u32>,
    ) -> Self {
        if let Some(resolution) = resolution_hint {
            let pixel_size = 1.0 / (resolution as f32);

            let min_line_stride = pixel_size * 6.0;
            let min_line_size = pixel_size * 1.8;

            if line_stride < min_line_stride {
                let old_line_stride = line_stride;
                line_stride = min_line_stride;
                line_width = line_width * line_stride / old_line_stride;
            }

            if line_width < min_line_size {
                line_width = min_line_size;
            } else if line_width > line_stride - min_line_size {
                line_width = line_stride - min_line_size;
            }
        }

        Self::new(line_width, line_stride, cycle_offset)
    }

    /// Creates an iterator that returns all the lines that need to be drawn
    pub fn draw_iter(&self, cycle_position: f32) -> LinesIter {
        let cycle_position = (cycle_position + self.cycle_offset).rem_euclid(1.0);
        LinesIter {
            line_width: self.line_width,
            line_stride: self.line_stride,
            current_pos: (cycle_position - 1.0) * self.line_stride,
            num_rendered: 0,
            num_total: self.num_lines,
        }
    }

    /// Change the line width
    pub fn set_line_width(&mut self, line_width: f32) {
        self.line_width = line_width;
    }

    /// Returns the offset between two lines
    pub fn get_line_stride(&self) -> f32 {
        self.line_stride
    }
}

/// The properties of a line on the screen
#[derive(Debug, Clone)]
pub struct Line {
    /// The identifying number of the line.
    pub id: u16,
    /// The start position of the line.
    pub start: f32,
    /// The end position of the line.
    pub end: f32,
}

/// An iterator over evenly spaced lines.
pub struct LinesIter {
    line_width: f32,
    line_stride: f32,
    current_pos: f32,
    num_rendered: u16,
    num_total: u16,
}

impl Iterator for LinesIter {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_total == self.num_rendered {
            None
        } else {
            let line = Line {
                id: self.num_rendered,
                start: self.current_pos,
                end: self.current_pos + self.line_width,
            };

            self.current_pos += self.line_stride;
            self.num_rendered += 1;

            Some(line)
        }
    }
}
