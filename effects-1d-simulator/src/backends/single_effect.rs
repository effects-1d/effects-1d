use effects_1d_common::{
    color::{self, Color},
    effects::{BeatBasedEffect, FrameBufferRef},
    errors::RenderError,
    random::{EffectRng, Rng},
};
use effects_1d_runtime::SwitchTimer;

use rand::rngs::OsRng;
use tracing::{error, info};

use crate::{effect_backend::SimulationFramebuffer, run_simulation, EffectBackend};

/// Adds a simulator to the effect types
pub trait SimulateEffect {
    /// Runs a simulator that shows the given effect
    fn simulate();
}

struct SingleEffectBackend<T> {
    switch_requested: bool,
    switch_timer: SwitchTimer,
    running_effect: Option<T>,
}

impl<T> Default for SingleEffectBackend<T> {
    fn default() -> Self {
        Self {
            running_effect: None,
            switch_timer: SwitchTimer::new(30.0),
            switch_requested: false,
        }
    }
}

impl<T: BeatBasedEffect> EffectBackend for SingleEffectBackend<T>
where
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn render_next_frame(
        &mut self,
        data: &mut [color::RGB],
        elapsed_time_seconds: f32,
        beat: effects_1d_common::effects::BeatInfo,
    ) -> String {
        if self.switch_requested && beat.is_new_beat {
            info!("Effect switched.");
            self.running_effect = None;
            self.switch_requested = false;
        }

        let data_len = data.len();
        let t0 = std::time::Instant::now();
        let effect_state = loop {
            data.fill(color::RGB::zero());
            let mut framebuffer = SimulationFramebuffer::new(data);
            let effect = self.running_effect.get_or_insert_with(|| {
                T::init(Some(data_len as u32), beat.next_full_beat().current)
            });

            match effect.render_frame(&mut framebuffer, elapsed_time_seconds, beat) {
                Ok(effect_state) => {
                    if self.switch_requested {
                        // Simulate some fade-out transition
                        framebuffer.fade(beat.fractional);
                    }
                    break effect_state;
                }
                Err(RenderError::EffectOver) => {
                    info!("Effect is over. Restarting ...");
                    self.running_effect = None;
                    continue;
                }
                Err(e) => error!("Effect failed: {:?}", e),
            };
        };
        let render_duration = t0.elapsed();

        let result = format!(
            "Compute Time: {:.01?}\n{:#?}\n{:#?}\n\nState:\n{:#?}",
            render_duration,
            beat,
            effect_state,
            self.running_effect.as_mut().unwrap(),
        );

        if self
            .switch_timer
            .update(elapsed_time_seconds, beat, effect_state.idle)
            && !self.switch_requested
        {
            info!("Request effect switching ...");
            self.switch_requested = true;
        }

        result
    }

    fn render_settings_gui(&mut self, ui: &mut three_d::egui::Ui) {
        use three_d::egui::*;

        let mut effect_duration = self.switch_timer.get_effect_duration();
        ui.add(
            Slider::new(&mut effect_duration, 5.0..=600.0)
                .suffix(" s")
                .text("Effect Duration")
                .logarithmic(true),
        );
        self.switch_timer.set_effect_duration(effect_duration);
    }
}

impl<T> SimulateEffect for T
where
    T: BeatBasedEffect,
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn simulate() {
        EffectRng::seed(OsRng.gen());

        let effect_renderer: SingleEffectBackend<T> = Default::default();
        run_simulation(effect_renderer);
    }
}
