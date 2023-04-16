use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

pub mod laser_sim;
pub mod ledstrip_sim;

#[derive(Component)]
pub struct SimWidget {
    /// Position of the widget's top left point
    ///  - (0,0) would be the in the top left corner
    ///  - (0.5,0.5) would mean the top left corner of the widget is in the center of the screen
    pub rel_position: Vec2,
    /// Size of the widget
    pub rel_size: Vec2,
}

impl SimWidget {
    pub fn compute_transform(&self, window_size: Vec2) -> Transform {
        Transform::default()
            .with_scale((window_size * self.rel_size).extend(0.))
            .with_translation(
                (window_size * (self.rel_position + self.rel_size * 0.5 - 0.5)).extend(0.),
            )
    }
}

#[derive(Bundle)]
pub struct SimWidgetBundle<M: WidgetMaterial> {
    pub widget: SimWidget,
    pub content: MaterialMesh2dBundle<M>,
}

impl<M: WidgetMaterial> SimWidgetBundle<M> {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<M>,
        window: &Window,
        rel_position: Vec2,
        rel_size: Vec2,
    ) -> Self {
        let window_size = Vec2::new(window.width(), window.height());

        let widget = SimWidget {
            rel_position,
            rel_size,
        };

        let transform = widget.compute_transform(window_size);

        Self {
            widget,
            content: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform,
                material: materials.add(M::new(rel_size, window_size)),
                ..default()
            },
        }
    }
}

pub trait WidgetMaterial: Material2d {
    fn new(rel_size: Vec2, window_size: Vec2) -> Self;
    fn update_window_size(&mut self, window_size: Vec2);
}
