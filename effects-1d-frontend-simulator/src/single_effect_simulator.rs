use std::time::Instant;

use effects_1d_common::effects::{BeatBasedEffect, BeatInfo, FrameBufferRef};

use crate::{effect_renderer::SimulationFramebuffer, run_simulation, EffectRenderer};

/// Adds a simulator to the effect types
pub trait SimulateEffect {
    /// Runs a simulator that shows the given effect
    fn simulate();
}

const BPM: f32 = 109.0;

impl<T> SimulateEffect for T
where
    T: BeatBasedEffect,
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn simulate() {
        let mut running_effect = None;
        let mut beat: BeatInfo = BeatInfo::zero();

        let effect_renderer = EffectRenderer::new(Box::new(move |data, time| {
            beat.progress(BPM * time.delta_seconds() / 60.0);

            let data_len = data.len();
            let t0 = Instant::now();
            let mut framebuffer = SimulationFramebuffer::new(data);
            let effect = running_effect.get_or_insert_with(|| Self::init(Some(data_len as u32)));
            let effect_state = effect
                .render_frame(&mut framebuffer, time.delta_seconds(), beat.clone())
                .unwrap();
            let render_duration = t0.elapsed();

            let result = format!(
                "{:#?}\nCompute Time: {:.01?}\n{:#.01?}\n{:#?}",
                effect, render_duration, beat, effect_state
            );

            if effect_state.over {
                running_effect = None;
            }

            result
        }));
        run_simulation(effect_renderer)
    }
}
