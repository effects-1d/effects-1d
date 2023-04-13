use bevy::prelude::*;

use std::time::Instant;

use effects_1d_common::{
    effects::{BeatBasedEffect, BeatInfo, FrameBufferRef},
    errors::RenderError,
};

use crate::{effect_renderer::SimulationFramebuffer, run_simulation, EffectRenderer};

/// Adds a simulator to the effect types
pub trait SimulateEffect {
    /// Runs a simulator that shows the given effect
    fn simulate();
}

const BPM: f32 = 120.0;

impl<T> SimulateEffect for T
where
    T: BeatBasedEffect,
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn simulate() {
        let mut running_effect = None;
        let mut beat: BeatInfo = BeatInfo::zero();

        let effect_renderer = EffectRenderer::new(Box::new(move |data, time| {
            let data_len = data.len();
            let t0 = Instant::now();
            let effect_state = loop {
                data.fill(0);
                let mut framebuffer = SimulationFramebuffer::new(data);
                let effect = running_effect.get_or_insert_with(|| {
                    Self::init(Some(data_len as u32), beat.next_full_beat().current)
                });

                match effect.render_frame(&mut framebuffer, time.delta_seconds(), beat.clone()) {
                    Ok(effect_state) => break effect_state,
                    Err(RenderError::EffectOver) => {
                        running_effect = None;
                        continue;
                    }
                    Err(e) => error!("Effect failed: {:?}", e),
                };
            };
            let render_duration = t0.elapsed();

            let result = format!(
                "{:#?}\nCompute Time: {:.01?}\n{:#.01?}\n{:#?}",
                running_effect.as_mut().unwrap(),
                render_duration,
                beat,
                effect_state
            );

            beat.progress(BPM * time.delta_seconds() / 60.0);

            if effect_state.idle && beat.is_new_beat {
                let rng: f32 = rand::random();
                if rng < 0.1 {
                    running_effect = None;
                }
            }

            result
        }));
        run_simulation(effect_renderer)
    }
}
