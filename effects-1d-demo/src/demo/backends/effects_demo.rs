use crate::demo::effect_collection::{DemoEffect, DemoEffectInstance};

use web_time::Instant;

use effects_1d::{
    BinaryEffect, BinaryRgbEffect, CalibrationMonochromeEffect, CalibrationRgbEffect,
    MonochromeEffect, RgbEffect,
};
use effects_1d_common::{
    color::{self, Color},
    errors::RenderError,
};
use effects_1d_simulator::{EffectBackend, SimulationFramebuffer};

pub struct EffectsDemoBackend {
    switch_requested: bool,
    switch_in_progress: bool,
    running_effect: Option<DemoEffectInstance>,
    desired_effect: DemoEffect,
}

impl Default for EffectsDemoBackend {
    fn default() -> Self {
        Self {
            running_effect: None,
            switch_requested: false,
            switch_in_progress: false,
            desired_effect: DemoEffect::Rgb(RgbEffect::SegmentedSnake),
        }
    }
}

impl EffectBackend for EffectsDemoBackend {
    fn render_next_frame(
        &mut self,
        data: &mut [color::RGB],
        elapsed_time_seconds: f32,
        beat: effects_1d_common::effects::BeatInfo,
    ) -> String {
        if self.switch_in_progress && beat.is_new_beat {
            tracing::info!("Effect switched.");
            self.running_effect = None;
            self.switch_in_progress = false;
            self.switch_requested = false;
        }
        if self.switch_requested && beat.is_new_beat {
            tracing::info!("Switching effect ...");
            self.switch_in_progress = true;
            self.switch_requested = false;
        }

        let data_len = data.len();
        let t0 = Instant::now();
        let effect_state = loop {
            data.fill(color::RGB::zero());
            let mut framebuffer = SimulationFramebuffer::new(data);
            let effect = self.running_effect.get_or_insert_with(|| {
                self.desired_effect
                    .create(Some(data_len as u32), beat.next_full_beat().current)
            });

            match effect.render_frame(&mut framebuffer, elapsed_time_seconds, beat) {
                Ok(effect_state) => {
                    if self.switch_in_progress {
                        // Simulate some fade-out transition
                        framebuffer.fade(beat.fractional);
                    }
                    break effect_state;
                }
                Err(RenderError::EffectOver) => {
                    tracing::info!("Effect is over. Restarting ...");
                    self.running_effect = None;
                    continue;
                }
                Err(e) => tracing::error!("Effect failed: {:?}", e),
            };
        };
        let render_duration = t0.elapsed();

        let result = format!(
            "Compute Time: {:.01?}\n{:#?}\n{:#?}\n\n{:#?}",
            render_duration,
            beat,
            effect_state,
            self.running_effect.as_mut().unwrap(),
        );

        result
    }

    fn render_settings_gui(&mut self, ui: &mut three_d::egui::Ui) {
        use three_d::egui::*;

        let previous_desired_effect = self.desired_effect;

        ComboBox::from_id_salt("Effect")
            .selected_text(format!("{}", self.desired_effect))
            .width(250.0)
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);

                ui.label("2-Color");
                for &effect in BinaryEffect::available() {
                    ui.selectable_value(
                        &mut self.desired_effect,
                        DemoEffect::Binary(effect),
                        format!("{}", effect),
                    );
                }

                ui.add_space(12.0);

                ui.label("8-Color");
                for &effect in BinaryRgbEffect::available() {
                    ui.selectable_value(
                        &mut self.desired_effect,
                        DemoEffect::BinaryRgb(effect),
                        format!("{}", effect),
                    );
                }

                ui.add_space(12.0);

                ui.label("Mono");
                for &effect in MonochromeEffect::available() {
                    ui.selectable_value(
                        &mut self.desired_effect,
                        DemoEffect::Monochrome(effect),
                        format!("{}", effect),
                    );
                }

                ui.add_space(12.0);

                ui.label("RGB");
                for &effect in RgbEffect::available() {
                    ui.selectable_value(
                        &mut self.desired_effect,
                        DemoEffect::Rgb(effect),
                        format!("{}", effect),
                    );
                }

                ui.add_space(12.0);

                ui.label("Calibration");
                for &effect in CalibrationMonochromeEffect::available() {
                    ui.selectable_value(
                        &mut self.desired_effect,
                        DemoEffect::CalibrationMono(effect),
                        format!("{}", effect),
                    );
                }
                for &effect in CalibrationRgbEffect::available() {
                    ui.selectable_value(
                        &mut self.desired_effect,
                        DemoEffect::CalibrationRgb(effect),
                        format!("{}", effect),
                    );
                }
            });

        if self.desired_effect != previous_desired_effect {
            self.switch_requested = true;
        }

        if ui.button("↺ Restart Effect").clicked() {
            self.switch_requested = true;
        }
    }
}
