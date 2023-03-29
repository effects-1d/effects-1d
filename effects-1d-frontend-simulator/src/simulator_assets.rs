use std::ops::{Add, Mul, Sub};

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

#[derive(Component)]
pub struct SimWidget {
    /// Position of the widget's top left point
    ///  - [0,0] would be the in the top left corner
    ///  - [0.5,0.5] would mean the top left corner of the widget is in the center of the screen
    pub rel_position: Vec2,
    /// Size of the widget
    pub rel_size: Vec2,
}

impl SimWidget {
    pub fn compute_transform(&self, window_size: Vec2) -> Transform {
        Transform::default()
            .with_scale(window_size.mul(self.rel_size).extend(0.))
            .with_translation(
                window_size
                    .mul(self.rel_position.add(self.rel_size.mul(0.5)).sub(0.5))
                    .extend(0.),
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

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a71be379-cac5-4204-8dbd-33982a9e1a60"]
pub struct LaserSimMaterial {}

impl Material2d for LaserSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/laser.wgsl".into()
    }
}

impl WidgetMaterial for LaserSimMaterial {
    fn new(_rel_size: Vec2, window_size: Vec2) -> Self {
        let mut this = Self {
            // effect_data: vec![0],
            // widget_size: default(),
            // rel_size,
        };
        this.update_window_size(window_size);
        this
    }

    fn update_window_size(&mut self, _window_size: Vec2) {
        //self.widget_size = self.rel_size.mul(window_size);
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "69fdd51f-11e6-4ffd-9ebf-2badfdd98f37"]
pub struct LedStripSimMaterial {
    #[storage(0, read_only)]
    pub effect_data: Vec<u32>,

    #[uniform(1)]
    pub widget_size: Vec2,

    pub rel_size: Vec2,
}

impl Material2d for LedStripSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/led_strip.wgsl".into()
    }
}

impl WidgetMaterial for LedStripSimMaterial {
    fn new(rel_size: Vec2, window_size: Vec2) -> Self {
        let mut this = Self {
            effect_data: vec![0],
            widget_size: default(),
            rel_size,
        };
        this.update_window_size(window_size);
        this
    }

    fn update_window_size(&mut self, window_size: Vec2) {
        self.widget_size = self.rel_size.mul(window_size);
    }
}

/*
pub fn compute_ledstrip_position(window_width: f32, window_height: f32) -> Transform {
    Transform::default()
        .with_scale(Vec3::new(window_width, window_height * 0.05, 0.))
        .with_translation(Vec3::new(0., window_height * (0.5 - 0.05), 0.))
}

pub fn compute_laser_position(window_width: f32, window_height: f32) -> Transform {
    Transform::default()
        .with_scale(Vec3::new(window_width, window_height * 0.9, 0.))
        .with_translation(Vec3::new(0., -window_height * 0.05, 0.))
}
*/
