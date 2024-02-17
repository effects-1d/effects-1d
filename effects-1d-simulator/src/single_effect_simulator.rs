use effects_1d_common::{
    color::{self, Color},
    effects::{BeatBasedEffect, FrameBufferRef},
    errors::RenderError,
    random::{EffectRng, Rng},
};
use effects_1d_runtime::SwitchTimer;

use rand::rngs::OsRng;
use tracing::{error, info};

use crate::{effect_renderer::SimulationFramebuffer, run_simulation, EffectRenderer};

/// Adds a simulator to the effect types
pub trait SimulateEffect {
    /// Runs a simulator that shows the given effect
    fn simulate();
}

impl<T> SimulateEffect for T
where
    T: BeatBasedEffect,
    for<'a> SimulationFramebuffer<'a>: FrameBufferRef<<T as BeatBasedEffect>::Color>,
{
    fn simulate() {
        EffectRng::seed(OsRng.gen());

        let mut running_effect = None;
        let mut switch_timer = SwitchTimer::new(10.0);
        let mut switch_requested = false;

        let effect_renderer = EffectRenderer::new(Box::new(move |data, delta_seconds, beat| {
            if switch_requested && beat.is_new_beat {
                info!("Effect switched.");
                running_effect = None;
                switch_requested = false;
            }

            let data_len = data.len();
            let t0 = std::time::Instant::now();
            let effect_state = loop {
                data.fill(color::RGB::zero());
                let mut framebuffer = SimulationFramebuffer::new(data);
                let effect = running_effect.get_or_insert_with(|| {
                    Self::init(Some(data_len as u32), beat.next_full_beat().current)
                });

                match effect.render_frame(&mut framebuffer, delta_seconds, beat) {
                    Ok(effect_state) => {
                        if switch_requested {
                            // Simulate some fade-out transition
                            framebuffer.fade(beat.fractional);
                        }
                        break effect_state;
                    }
                    Err(RenderError::EffectOver) => {
                        info!("Effect is over. Restarting ...");
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

            if switch_timer.update(delta_seconds, beat, effect_state.idle) && !switch_requested {
                info!("Request effect switching ...");
                switch_requested = true;
            }

            result
        }));
        run_simulation(effect_renderer)
    }
}
