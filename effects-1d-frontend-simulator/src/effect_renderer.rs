use bevy::prelude::*;

use rand::Rng;

// TODO move rendering in separate struct so it can keep its state
#[derive(Resource)]
pub struct EffectRenderer {
    old_color: Vec3,
    new_color: Vec3,
    percent: f32,
}

fn random_color() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
    )
}

const SPEED: f32 = 0.5;

fn color_to_u32(color: Vec3) -> u32 {
    let r = (color.x * 256.0).clamp(0., 255.) as u32;
    let g = (color.y * 256.0).clamp(0., 255.) as u32;
    let b = (color.z * 256.0).clamp(0., 255.) as u32;

    (r << 0) | (g << 8) | (b << 16)
}

impl EffectRenderer {
    pub fn new() -> Self {
        Self {
            old_color: Vec3::new(0., 0., 0.),
            new_color: random_color(),
            percent: 0.,
        }
    }

    pub fn render_next_frame(&mut self, framebuffer: &mut [u32], time: &Time) {
        self.percent += time.delta_seconds() * SPEED;
        if self.percent > 1. {
            self.percent -= 1.;
            self.old_color = self.new_color;
            self.new_color = random_color();
        }

        let pos_max = (framebuffer.len() - 1) as f32;
        for (pos, val) in framebuffer.iter_mut().enumerate() {
            let pos = pos as f32 / pos_max;
            let color = if pos <= self.percent {
                self.new_color
            } else {
                self.old_color
            };

            *val = color_to_u32(color);
        }
    }
}
