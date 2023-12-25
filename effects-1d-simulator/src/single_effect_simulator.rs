use bevy::{prelude::*, utils::Instant};

use effects_1d_common::{
    color::{self, Color},
    effects::{BeatBasedEffect, BeatInfo, FrameBufferRef},
    errors::RenderError,
    random::{EffectRng, Rng},
};
use rand::rngs::OsRng;

use crate::{effect_renderer::SimulationFramebuffer, run_simulation, EffectRenderer};

/// Adds a simulator to the effect types
pub trait SimulateEffect {
    /// Runs a simulator that shows the given effect
    fn simulate();
}

const BPM: f32 = 125.0;

impl<T> SimulateEffect for T
where
    T: BeatBasedEffect,
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn simulate() {
        EffectRng::seed(OsRng.gen());

        let mut running_effect = None;
        let mut beat: BeatInfo = BeatInfo::zero();
        //let mut first_idle = None;

        let effect_renderer = EffectRenderer::new(Box::new(move |data, time| {
            let data_len = data.len();
            let t0 = Instant::now();
            let effect_state = loop {
                data.fill(color::RGB::zero());
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
                "{:#?}\nCompute Time: {:.01?}\n{:#?}\n{:#?}",
                running_effect.as_mut().unwrap(),
                render_duration,
                beat,
                effect_state
            );

            beat.progress(BPM * time.delta_seconds() / 60.0);

            /*
            if let Some(since) = first_idle.as_mut() {
                *since += time.delta_seconds();
            }
            if effect_state.idle && beat.is_new_beat {
                if let Some(since) = first_idle {
                    if since > 5.0 {
                        let rng: f32 = rand::random();
                        if rng < 0.1 {
                            running_effect = None;
                            first_idle = None;
                        }
                    }
                } else {
                    first_idle = Some(0.0);
                }
            }
            */

            result
        }));
        run_simulation(effect_renderer)
    }
}
