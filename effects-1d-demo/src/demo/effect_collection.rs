use core::fmt::Display;

use effects_1d::{
    BinaryEffect, BinaryEffectInstance, BinaryRgbEffect, BinaryRgbEffectInstance,
    CalibrationMonochromeEffect, CalibrationMonochromeEffectInstance, CalibrationRgbEffect,
    CalibrationRgbEffectInstance, MonochromeEffect, MonochromeEffectInstance, RgbEffect,
    RgbEffectInstance,
};
use effects_1d_common::{
    effects::{BeatInfo, EffectState},
    errors::RenderError,
};
use effects_1d_simulator::SimulationFramebuffer;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DemoEffect {
    Binary(BinaryEffect),
    Monochrome(MonochromeEffect),
    BinaryRgb(BinaryRgbEffect),
    Rgb(RgbEffect),
    CalibrationMono(CalibrationMonochromeEffect),
    CalibrationRgb(CalibrationRgbEffect),
}

#[derive(Debug)]
pub enum DemoEffectInstance {
    Binary(BinaryEffectInstance),
    Monochrome(MonochromeEffectInstance),
    BinaryRgb(BinaryRgbEffectInstance),
    Rgb(RgbEffectInstance),
    CalibrationMono(CalibrationMonochromeEffectInstance),
    CalibrationRgb(CalibrationRgbEffectInstance),
}

impl Display for DemoEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DemoEffect::Binary(effect) => write!(f, "2-Color - {}", effect),
            DemoEffect::BinaryRgb(effect) => write!(f, "8-Color - {}", effect),
            DemoEffect::Monochrome(effect) => write!(f, "Mono - {}", effect),
            DemoEffect::Rgb(effect) => write!(f, "RGB - {}", effect),
            DemoEffect::CalibrationMono(effect) => write!(f, "Calibration - {}", effect),
            DemoEffect::CalibrationRgb(effect) => write!(f, "Calibration - {}", effect),
        }
    }
}

impl DemoEffect {
    pub fn create(self, resolution_hint: Option<u32>, start_beat: i32) -> DemoEffectInstance {
        match self {
            DemoEffect::Binary(effect) => {
                DemoEffectInstance::Binary(effect.create(resolution_hint, start_beat))
            }
            DemoEffect::Monochrome(effect) => {
                DemoEffectInstance::Monochrome(effect.create(resolution_hint, start_beat))
            }
            DemoEffect::BinaryRgb(effect) => {
                DemoEffectInstance::BinaryRgb(effect.create(resolution_hint, start_beat))
            }
            DemoEffect::Rgb(effect) => {
                DemoEffectInstance::Rgb(effect.create(resolution_hint, start_beat))
            }
            DemoEffect::CalibrationMono(effect) => {
                DemoEffectInstance::CalibrationMono(effect.create(resolution_hint, start_beat))
            }
            DemoEffect::CalibrationRgb(effect) => {
                DemoEffectInstance::CalibrationRgb(effect.create(resolution_hint, start_beat))
            }
        }
    }
}

impl DemoEffectInstance {
    pub fn render_frame(
        &mut self,
        framebuffer: &mut SimulationFramebuffer,
        d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        match self {
            DemoEffectInstance::Binary(effect) => {
                effect.as_effect().render_frame(framebuffer, d_t, beat)
            }
            DemoEffectInstance::Monochrome(effect) => {
                effect.as_effect().render_frame(framebuffer, d_t, beat)
            }
            DemoEffectInstance::BinaryRgb(effect) => {
                effect.as_effect().render_frame(framebuffer, d_t, beat)
            }
            DemoEffectInstance::Rgb(effect) => {
                effect.as_effect().render_frame(framebuffer, d_t, beat)
            }
            DemoEffectInstance::CalibrationMono(effect) => {
                effect.as_effect().render_frame(framebuffer, d_t, beat)
            }
            DemoEffectInstance::CalibrationRgb(effect) => {
                effect.as_effect().render_frame(framebuffer, d_t, beat)
            }
        }
    }
}
