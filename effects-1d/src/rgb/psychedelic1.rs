/*
Loosely based on https://www.shadertoy.com/view/mslyDf
and https://youtu.be/f4s1h2YETNY
*/

#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, palette::Srgb},
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

use glam::f32::{Vec2, Vec3};

use core::f32::consts::PI;

#[derive(Debug)]
pub struct Psychedelic1 {
    cycle: MultiBeatCycle,
}

fn palette(t: f32) -> Vec3 {
    const A: Vec3 = Vec3::new(0.892, 0.725, 0.000);
    const B: Vec3 = Vec3::new(0.878, 0.278, 0.968);
    const C: Vec3 = Vec3::new(0.332, 0.518, 0.545);
    const D: Vec3 = Vec3::new(2.440, 5.043, 0.732);

    let i = 6.28318 * (C * t + D);
    let j = Vec3::new(i.x.cos(), i.y.cos(), i.z.cos());

    return A + B * j;
}

impl ConstructibleBeatBasedEffect for Psychedelic1 {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(4, start_beat),
        }
    }
}

impl BeatBasedEffect for Psychedelic1 {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let t = self.cycle.cycle_number() as f32 + self.cycle.cycle_progress();
        let t = t * PI / 2.;

        let bufsize = framebuffer.len().saturating_sub(1);
        framebuffer.set_pixels(&mut |pos| {
            let col = render_pixel(pos, bufsize, t);
            Srgb::<f32>::new(col.x, col.y, col.z).into_format()
        });
        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}

fn render_pixel(x: u32, resolution: u32, t: f32) -> Vec3 {
    let x = (x as f32 * 2.0) / (resolution as f32 - 1.0) - 1.0; // Normalize to [-1, 1]

    let y = 0.5 * (t * 0.1).sin();

    let mut uv = Vec2::new(x * 1.3, y);

    let uv0 = uv;
    let mut final_color = Vec3::ZERO;

    let uv0_len = uv0.length();
    let uv0_lenexp = (-uv0_len).exp() * 0.7;

    for i in 0..3 {
        let i = i as f32;

        uv = (uv * 1.5).fract() - 0.5;

        let mut d = uv.length() * uv0_lenexp;

        let color_change_speed = 0.3;

        let col = palette(uv0_len + color_change_speed * (i + t));

        let frequency = 17.0;

        // Sidenote: the stroke width decreases with each iteration
        d = (d * frequency + t).sin() / (frequency - i * 5.0);

        d = d.abs();

        d = (0.0085 / d).powf(1.4);

        final_color += col * d;
    }

    final_color
}

/* Shadertoy code:

vec3 palette(float t) {
    vec3 a = vec3(0.892, 0.725, 0.000);
    vec3 b = vec3(0.878, 0.278, 0.968);
    vec3 c = vec3(0.332, 0.518, 0.545);
    vec3 d = vec3(2.440, 5.043, 0.732);

    return a + b*cos(6.28318*(c*t+d));
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // fragCoord.y = fragCoord.y * 1. + iResolution.y / 2. + iResolution.y / 4. * sin(iTime * 0.1);
    // vec2 uv = (fragCoord * 2.0 - iResolution.xy) / iResolution.x;

    float x = (fragCoord.x * 2.0) / iResolution.x - 1.0; // Normalize to [-1, 1]
    float y = 0.5 * sin(iTime * 0.1);
    //y += (fragCoord.y * 2.0) / iResolution.y;

    vec2 uv = vec2(x * 1.3,y) ;

    vec2 uv0 = uv;
    vec3 finalColor = vec3(0.0);

    float uv0_len = length(uv0);
    float uv0_lenexp = exp(-uv0_len) * 0.7;

    for (float i = 0.0; i < 3.0; i++) {
        uv = fract(uv * 1.5) - 0.5;

        float d = length(uv) * uv0_lenexp;

        float colorChangeSpeed = 0.3;

        vec3 col = palette(uv0_len + colorChangeSpeed * (i + iTime));

        float frequency = 17.0;

        // Sidenote: the stroke width decreases with each iteration
        d = sin(d * frequency + iTime) / (frequency - i * 5.0);

        d = abs(d);

        d = pow(0.0085 / d, 1.4);

        finalColor += col * d;
    }

    fragColor = vec4(finalColor, 1.0);
}
*/
