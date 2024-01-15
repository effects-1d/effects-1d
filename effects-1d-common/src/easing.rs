use crate::lerp::Lerp;

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct BezierEasingStep {
    pub x: f32,
    pub y: f32,
}

const BEZIER_EASING_STEPS: usize = 32;

/// Cubic Bezier based easing curve.
pub struct BezierEasing {
    steps: [BezierEasingStep; BEZIER_EASING_STEPS],
}

impl BezierEasing {
    /// Creates a new Cubic Bezier based easing.
    ///
    /// # Arguments
    ///
    /// * `x0`,`y0` - The coordinates of the first control point. This control point determines the start of the easing.
    /// * `x1`,`y1` - The coordinates of the second control point. This control point determines the end of the easing.
    ///
    /// The `x` coordinates of the control points must lie in the interval of `[0.0, 1.0]`. Values outside of that range will get clipped to it.
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        assert!(BEZIER_EASING_STEPS >= 2);

        let x0 = x0.clamp(0.0, 1.0);
        let x1 = x1.clamp(0.0, 1.0);

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

macro_rules! easing_function {
    (@small_svg ($x0:literal, $y0:literal, $x1:literal, $y1:literal)) => {
      //concat!(r#"<svg width="26" height="18" xmlns="http://www.w3.org/2000/svg" style="position: relative; top: 0.2em;"><rect width="100%" height="100%" style="fill:darkgray;fill-opacity:0.5" /><g transform="scale(1 -1) translate(1, -17)"><path d="M 0 0 c "#, $x0, ",", $y0, " ", $x1, ",", $y1, r#" 24,16" style="fill:none; stroke:darkred; stroke-width:1.5px; stroke-opacity:1.0; stroke-linecap:round"></path></g></svg>"#)
        concat!(r#"<svg width="26" height="18" xmlns="http://www.w3.org/2000/svg" style="position: relative; top: 0.2em;"><rect width="100%" height="100%" style="fill:darkgray;fill-opacity:0.5" /><g transform="scale(24 -16) translate(0.04166666, -1.0625)"><path d="M 0 0 c "#, $x0, ",", $y0, " ", $x1, ",", $y1, r#" 1,1" style="fill:none; stroke:darkred; stroke-width:0.07px; stroke-opacity:1.0; stroke-linecap:square"></path></g></svg>"#)
    };
    (@large_svg ($x0:literal, $y0:literal, $x1:literal, $y1:literal)) => {
        concat!(r#"<svg width="157" height="105" xmlns="http://www.w3.org/2000/svg">
        <rect x="1" y="1" width="155" height="103" style="fill:darkgray;stroke:black;stroke-width:2;fill-opacity:0.5;stroke-opacity:0.7" />
        <g transform="scale(155 -102) translate(0.00645, -1.015)">
        <path
            d="M 0 0 c "#, $x0, ",", $y0, " ", $x1, ",", $y1, r#" 1,1"
            style="fill:none; stroke:darkred; stroke-width:.025px; stroke-opacity:1.0; stroke-linecap:square"
        ></path>
        </g>
        </svg>"#)
    };
    (@inner $name:ident, ($x0:literal, $y0:literal, $x1:literal, $y1:literal), $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            pub fn [<$name:snake>]() -> BezierEasing {
                BezierEasing::new($x0, $y0, $x1, $y1)
            }
        }
    };
    ($name:ident, ($x0:literal, $y0:literal, $x1:literal, $y1:literal), $doc:expr) => {
        easing_function!(@inner $name, ($x0, $y0, $x1, $y1), concat!(
            easing_function!(@small_svg ($x0, $y0, $x1, $y1)),
            ", ", $doc, ".",
            "\n\n", "Equivalent to [`BezierEasing::new(", $x0, ", ", $y0, ", ", $x1, ", ", $y1, ")`](BezierEasing::new).",
            "\n\n", easing_function!(@large_svg ($x0, $y0, $x1, $y1)),
        ));
    };
    (@easings_net $name:ident, ($x0:literal, $y0:literal, $x1:literal, $y1:literal), $formula:expr) => {
        easing_function!($name, ($x0, $y0, $x1, $y1), concat!(
            "taken from [easings.net](https://easings.net/#", stringify!($name), ")"
        ));

        ::paste::paste! {
            #[cfg(test)]
            #[test]
            fn [< test_ $name:snake >](){
                #[allow(unused_imports)]
                use core::f32::consts::PI;

                let formula: fn(f32)->f32 = $formula;

                let easing = [< $name:snake >]();

                const STEPS: u16 = 512;
                for step in 0..=STEPS {
                    let pos = f32::from(step) / f32::from(STEPS);

                    let expected = formula(pos);
                    let actual = easing.evaluate(pos);

                    ::approx::assert_abs_diff_eq!(actual, expected, epsilon=0.035);
                }
            }
        }
    };
}

// Basic easing function
easing_function!(
    ease,
    (0.25, 0.1, 0.25, 1.0),
    "basic easing function for movement"
);

// Taken from https://easings.net.
easing_function!(@easings_net easeInSine, (0.12, 0.0, 0.39, 0.0), |x| 1.0 - ((x * PI) / 2.0).cos());
easing_function!(@easings_net easeOutSine, (0.61, 1.0, 0.88, 1.0), |x| ((x * PI) / 2.0).sin());
easing_function!(@easings_net easeInOutSine, (0.37, 0.0, 0.63, 1.0), |x| -((x * PI).cos() - 1.0) / 2.0);
easing_function!(@easings_net easeInQuad, (0.11, 0.0, 0.5, 0.0), |x| x * x);
easing_function!(@easings_net easeOutQuad, (0.5, 1.0, 0.89, 1.0), |x| 1.0 - (1.0 - x) * (1.0 - x));
easing_function!(@easings_net easeInOutQuad, (0.45, 0.0, 0.55, 1.0), |x| if x < 0.5 {2.0 * x * x} else {1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0});
easing_function!(@easings_net easeInCubic, (0.32, 0.0, 0.67, 0.0), |x| x * x * x);
easing_function!(@easings_net easeOutCubic, (0.33, 1.0, 0.68, 1.0), |x| 1.0 - (1.0 - x).powf(3.0));
easing_function!(@easings_net easeInOutCubic, (0.65, 0.0, 0.35, 1.0), |x| if x < 0.5 {4.0 * x * x * x} else {1.0 - (-2.0 * x + 2.0).powf(3.0) / 2.0});
easing_function!(@easings_net easeInQuart, (0.5, 0.0, 0.75, 0.0), |x| x * x * x * x);
easing_function!(@easings_net easeOutQuart, (0.25, 1.0, 0.5, 1.0), |x| 1.0 - (1.0 - x).powf(4.0));
easing_function!(@easings_net easeInOutQuart, (0.76, 0.0, 0.24, 1.0), |x| if x < 0.5 {8.0 * x * x * x * x} else {1.0 - (-2.0 * x + 2.0).powf(4.0) / 2.0});
easing_function!(@easings_net easeInQuint, (0.64, 0.0, 0.78, 0.0), |x| x * x * x * x * x);
easing_function!(@easings_net easeOutQuint, (0.22, 1.0, 0.36, 1.0), |x| 1.0 - (1.0 - x).powf(5.0));
easing_function!(@easings_net easeInOutQuint, (0.83, 0.0, 0.17, 1.0), |x| if x < 0.5 {16.0 * x * x * x * x * x} else {1.0 - (-2.0 * x + 2.0).powf(5.0) / 2.0});
// - name: easeInExpo
//   css:  cubic-bezier(0.7, 0, 0.84, 0)
//   maths: |-2
//     return x === 0 ? 0 : Math.pow(2, 10 * x - 10);
// - name: easeOutExpo
//   css:  cubic-bezier(0.16, 1, 0.3, 1)
//   maths: |-2
//     return x === 1 ? 1 : 1 - Math.pow(2, -10 * x);
// - name: easeInOutExpo
//   css:  cubic-bezier(0.87, 0, 0.13, 1)
//   maths: |-2
//     return x === 0
//       ? 0
//       : x === 1
//       ? 1
//       : x < 0.5 ? Math.pow(2, 20 * x - 10) / 2
//       : (2 - Math.pow(2, -20 * x + 10)) / 2;
// - name: easeInCirc
//   css:  cubic-bezier(0.55, 0, 1, 0.45)
//   maths: |-2
//     return 1 - Math.sqrt(1 - Math.pow(x, 2));
// - name: easeOutCirc
//   css:  cubic-bezier(0, 0.55, 0.45, 1)
//   maths: |-2
//     return Math.sqrt(1 - Math.pow(x - 1, 2));
// - name: easeInOutCirc
//   css:  cubic-bezier(0.85, 0, 0.15, 1)
//   maths: |-2
//     return x < 0.5
//       ? (1 - Math.sqrt(1 - Math.pow(2 * x, 2))) / 2
//       : (Math.sqrt(1 - Math.pow(-2 * x + 2, 2)) + 1) / 2;
// - name: easeInBack
//   css:  cubic-bezier(0.36, 0, 0.66, -0.56)
//   maths: |-2
//     const c1 = 1.70158;
//     const c3 = c1 + 1;

//     return c3 * x * x * x - c1 * x * x;
// - name: easeOutBack
//   css:  cubic-bezier(0.34, 1.56, 0.64, 1)
//   maths: |-2
//     const c1 = 1.70158;
//     const c3 = c1 + 1;

//     return 1 + c3 * Math.pow(x - 1, 3) + c1 * Math.pow(x - 1, 2);
// - name: easeInOutBack
//   css:  cubic-bezier(0.68, -0.6, 0.32, 1.6)
//   maths: |-2
//     const c1 = 1.70158;
//     const c2 = c1 * 1.525;

//     return x < 0.5
//       ? (Math.pow(2 * x, 2) * ((c2 + 1) * 2 * x - c2)) / 2
//       : (Math.pow(2 * x - 2, 2) * ((c2 + 1) * (x * 2 - 2) + c2) + 2) / 2;
