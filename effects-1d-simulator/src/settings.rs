use three_d::egui::*;

pub struct SimulatorSettings {
    pub resolution: usize,
    pub bpm: u16,
}

impl SimulatorSettings {
    pub fn render_gui(&mut self, ui: &mut Ui) {
        ui.add(
            Slider::new(&mut self.resolution, 32..=1024)
                .text("Resolution")
                .logarithmic(true),
        );
        ui.add(Slider::new(&mut self.bpm, 70..=160).text("BPM"));
    }
}

impl Default for SimulatorSettings {
    fn default() -> Self {
        Self {
            resolution: 1024,
            bpm: 125,
        }
    }
}
