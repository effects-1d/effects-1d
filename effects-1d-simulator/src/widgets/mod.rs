use effects_1d_common::effects::BeatInfo;
use three_d::egui::*;

/// A visual indicator of the current beat. Similar to a horizontal line.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // These are equivalent:
/// ui.add(BeatIndicator::default());
/// # });
/// ```
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct BeatIndicator {
    spacing: f32,
    line_width: f32,
    grow: f32,
    beat: BeatInfo,
}

impl BeatIndicator {
    pub fn new(beat: BeatInfo) -> Self {
        Self {
            spacing: 6.0,
            line_width: 4.0,
            grow: 0.0,
            beat,
        }
    }

    /// How much space we take up. The line is painted in the middle of this.
    #[inline]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// How big the line is.
    #[inline]
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }

    /// Extend each end of the separator line by this much.
    ///
    /// The default is to take up the available width/height of the parent.
    ///
    /// This will make the line extend outside the parent ui.
    #[inline]
    pub fn grow(mut self, extra: f32) -> Self {
        self.grow += extra;
        self
    }

    /// Contract each end of the separator line by this much.
    ///
    /// The default is to take up the available width/height of the parent.
    ///
    /// This effectively adds margins to the line.
    #[inline]
    pub fn shrink(mut self, shrink: f32) -> Self {
        self.grow -= shrink;
        self
    }
}

impl Widget for BeatIndicator {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            spacing,
            grow,
            beat,
            line_width,
        } = self;

        let available_space = ui.available_size_before_wrap();

        let size = vec2(available_space.x, spacing);

        let (rect, response) = ui.allocate_at_least(size, Sense::hover());

        if ui.is_rect_visible(response.rect) {
            let color = if beat.is_new_beat {
                Color32::RED
            } else {
                Color32::DARK_RED.gamma_multiply((1.0 - beat.fractional).powf(2.0))
            };
            let stroke = Stroke::new(line_width, color);

            let painter = ui.painter();
            painter.hline(
                (rect.left() - grow)..=(rect.right() + grow),
                painter.round_to_pixel(rect.center().y),
                stroke,
            );
        }

        response
    }
}
