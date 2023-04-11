use effects_1d_common::effects::{BeatBasedEffect, BeatInfo, FrameBufferRef};

use crate::{effect_renderer::SimulationFramebuffer, run_simulation, EffectRenderer};

/// Adds a simulator to the effect types
pub trait SimulatableEffect {
    /// Runs a simulator that shows the given effect
    fn simulate();
}

const BPM: f32 = 109.0;

impl<T> SimulatableEffect for T
where
    T: BeatBasedEffect,
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn simulate() {
        let mut running_effect = Self::init();
        let mut beat: BeatInfo = BeatInfo::zero();

        let effect_renderer = EffectRenderer::new(Box::new(move |data, time| {
            beat.progress(BPM * time.delta_seconds() / 60.0);

            let mut framebuffer = SimulationFramebuffer::new(data);
            let effect_state = running_effect
                .render_frame(&mut framebuffer, time.delta_seconds(), beat.clone())
                .unwrap();
            if effect_state.over {
                running_effect = Self::init();
            }
        }));
        run_simulation(effect_renderer)
    }
}
