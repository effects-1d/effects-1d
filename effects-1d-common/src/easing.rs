use crate::lerp::Lerp;

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct BezierEasingStep {
    pub x: f32,
    pub y: f32,
}

const BEZIER_EASING_STEPS: usize = 32;

/// Bezier based easing
pub struct BezierEasing {
    steps: [BezierEasingStep; BEZIER_EASING_STEPS],
}

impl BezierEasing {
    /// Creates a new Cubic Bezier based easing.
    ///
    /// # Arguments
    ///
    /// * `x0`,`y0` - The coordinates of the first control point. This control point determines the start of the easing.
    /// * `x0`,`y0` - The coordinates of the second control point. This control point determines the end of the easing.
    ///
    /// All control point coordinates must lie in the interval of `[0.0, 1.0]`. Values outside of that range will get clipped to it.
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        assert!(BEZIER_EASING_STEPS >= 2);

        let x0 = x0.clamp(0.0, 1.0);
        let y0 = y0.clamp(0.0, 1.0);
        let x1 = x1.clamp(0.0, 1.0);
        let y1 = y1.clamp(0.0, 1.0);

        let mut steps = [BezierEasingStep { x: 0.0, y: 0.0 }; BEZIER_EASING_STEPS];

        let pos_max = (steps.len() - 1) as f32;
        for (pos, step) in steps.iter_mut().enumerate() {
            let pos = pos as f32 / pos_max;

            let npos = 1.0 - pos;

            step.x = 3.0 * pos * npos * npos * x0 + 3.0 * pos * pos * npos * x1 + pos * pos * pos;
            step.y = 3.0 * pos * npos * npos * y0 + 3.0 * pos * pos * npos * y1 + pos * pos * pos;
        }

        Self { steps }
    }

    #[doc(hidden)]
    pub fn steps(&self) -> &[BezierEasingStep; BEZIER_EASING_STEPS] {
        &self.steps
    }

    /// Evaluates the easing at a given value.
    pub fn evaluate(&self, pos: f32) -> f32 {
        let mut range = 0..BEZIER_EASING_STEPS;

        while range.len() > 2 {
            let mid = range.start + range.len() / 2;
            if self.steps[mid].x >= pos {
                range.end = mid + 1;
            } else {
                range.start = mid;
            }
        }

        let val1 = self.steps[range.start];
        let val2 = self.steps[range.end - 1];
        let pos_rel = (pos - val1.x) / (val2.x - val1.x);

        val1.y.clamping_lerp(val2.y, pos_rel)
    }
}
