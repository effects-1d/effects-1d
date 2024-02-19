use effects_1d_common::effects::BeatInfo;

use crate::{settings::SimulatorSettings, EffectBackend};

pub struct EffectGUI {
    gui: three_d::GUI,
}

impl EffectGUI {
    pub fn new(context: &three_d::Context) -> Self {
        Self {
            gui: three_d::GUI::new(&context),
        }
    }

    pub fn update(
        &mut self,
        frame_input: &mut three_d::FrameInput,
        settings: &mut SimulatorSettings,
        backend: &mut dyn EffectBackend,
        effect_state: &str,
        beat: BeatInfo,
    ) -> three_d::Viewport {
        let mut panel_width = 0.0;

        self.gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Simulator Settings");
                    settings.render_gui(ui, beat);

                    ui.add_space(12.0);

                    ui.heading("Effect Settings");
                    backend.render_settings_gui(ui);

                    ui.add_space(12.0);

                    ui.heading("State");
                    ui.collapsing("Detailed State", |ui| {
                        ui.label(RichText::new(effect_state).monospace().small());
                    });
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        three_d::Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input
                .viewport
                .width
                .saturating_sub((panel_width * frame_input.device_pixel_ratio) as u32),
            height: frame_input.viewport.height,
        }
    }

    pub fn render(&self) {
        self.gui.render();
    }
}
