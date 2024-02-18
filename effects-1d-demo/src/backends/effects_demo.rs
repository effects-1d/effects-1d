use effects_1d::RgbEffects;
use effects_1d_common::{
    color::{self, Color},
    effects::{BeatBasedEffect, ConstructibleBeatBasedEffect, FrameBufferRef},
    errors::RenderError,
    random::{EffectRng, Rng},
};
use effects_1d_simulator::EffectBackend;

use tracing::{error, info};

pub struct EffectsDemoBackend {
    switch_requested: bool,
    switch_in_progress: bool,
    running_effect: Option<()>,
}

impl Default for EffectsDemoBackend {
    fn default() -> Self {
        Self {
            running_effect: None,
            switch_requested: false,
            switch_in_progress: false,
        }
    }
}

enum DemoEffects {
    RgbEffect(RgbEffects),
}

impl EffectBackend for EffectsDemoBackend {
    fn render_next_frame(
        &mut self,
        data: &mut [color::RGB],
        elapsed_time_seconds: f32,
        beat: effects_1d_common::effects::BeatInfo,
    ) -> String {
        // if self.switch_requested && beat.is_new_beat {
        //     info!("Effect switched.");
        //     self.running_effect = None;
        //     self.switch_requested = false;
        // }

        // let data_len = data.len();
        // let t0 = std::time::Instant::now();
        // let effect_state = loop {
        //     data.fill(color::RGB::zero());
        //     let mut framebuffer = SimulationFramebuffer::new(data);
        //     let effect = self.running_effect.get_or_insert_with(|| {
        //         T::init(Some(data_len as u32), beat.next_full_beat().current)
        //     });

        //     match effect.render_frame(&mut framebuffer, elapsed_time_seconds, beat) {
        //         Ok(effect_state) => {
        //             if self.switch_requested {
        //                 // Simulate some fade-out transition
        //                 framebuffer.fade(beat.fractional);
        //             }
        //             break effect_state;
        //         }
        //         Err(RenderError::EffectOver) => {
        //             info!("Effect is over. Restarting ...");
        //             self.running_effect = None;
        //             continue;
        //         }
        //         Err(e) => error!("Effect failed: {:?}", e),
        //     };
        // };
        // let render_duration = t0.elapsed();

        // let result = format!(
        //     "Compute Time: {:.01?}\n{:#?}\n{:#?}\n\nState:\n{:#?}",
        //     render_duration,
        //     beat,
        //     effect_state,
        //     self.running_effect.as_mut().unwrap(),
        // );

        // result

        "".to_string()
    }

    fn render_settings_gui(&mut self, ui: &mut three_d::egui::Ui) {
        use three_d::egui::*;

        ComboBox::from_label("Effect")
            .selected_text(format!("AAA"))
            .show_ui(ui, |ui| {});
    }
}
