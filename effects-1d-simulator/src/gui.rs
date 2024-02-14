pub struct EffectGUI {
    gui: three_d::GUI,
    // TODO: Add some actual values
    viewport_zoom: f64,
    scissor_zoom: f64,
}

impl EffectGUI {
    pub fn new(context: &three_d::Context) -> Self {
        Self {
            gui: three_d::GUI::new(&context),
            viewport_zoom: 1.0,
            scissor_zoom: 1.0,
        }
    }

    pub fn update(&mut self, frame_input: &mut three_d::FrameInput) -> three_d::Viewport {
        let mut panel_width = 0.0;

        self.gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut self.viewport_zoom, 0.01..=1.0).text("Viewport"));
                    ui.add(Slider::new(&mut self.scissor_zoom, 0.01..=1.0).text("Scissor"));
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        three_d::Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        }
    }

    pub fn render(&self) {
        self.gui.render();
    }
}
