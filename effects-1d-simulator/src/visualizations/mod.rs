use three_d::*;

mod ledstrip_sim_material;
pub use ledstrip_sim_material::LedStripSimMaterial;

pub struct SimWidget<M: Material> {
    /// Position of the widget's center point
    ///  - (0.5,0.5) would mean that the widget is in the center of the screen
    rel_position: Vec2,
    /// Size of the widget
    rel_size: Vec2,
    /// The actual rectangle to render
    gm: Gm<Rectangle, M>,
}

impl<M: Material> SimWidget<M> {
    pub fn new(
        context: &Context,
        rel_position: impl Into<Vec2>,
        rel_size: impl Into<Vec2>,
        material: M,
    ) -> Self {
        let gm = Gm::new(
            Rectangle::new(context, (0.0, 0.0), degrees(0.0), 0.0, 0.0),
            material,
        );
        Self {
            rel_position: rel_position.into(),
            rel_size: rel_size.into(),
            gm,
        }
    }

    pub fn update(&mut self, viewport: Viewport) {
        let width = viewport.width as f32 * self.rel_size.x;
        let height = viewport.height as f32 * self.rel_size.y;
        let x = viewport.width as f32 * self.rel_position.x;
        let y = viewport.height as f32 * self.rel_position.y;
        self.gm.geometry.set_center((x, y));
        self.gm.geometry.set_size(width, height);
    }

    pub fn obj(&self) -> &dyn Object {
        &self.gm
    }
}
