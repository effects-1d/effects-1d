#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResized},
};

mod effect_renderer;
mod fonts;
mod single_effect_simulator;
mod visualizations;

pub use effect_renderer::EffectRenderer;
use effects_1d_common::color::{self, palette::FromColor};
pub use single_effect_simulator::SimulateEffect;

use fonts::RobotoFontPlugin;
use visualizations::{
    laser_sim::{LaserSimMaterial, LaserSimPlugin},
    ledstrip_sim::{LedStripSimMaterial, LedStripSimPlugin},
    SimWidget, SimWidgetBundle, WidgetMaterial,
};

#[derive(Component)]
struct SimulationStateText;

#[derive(Component)]
struct EngineStateText;

/// Runs a simulation for the given effect/engine
pub fn run_simulation(effect_renderer: EffectRenderer) {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(effect_renderer)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "1D Effects Simulator".to_string(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        //.add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(LaserSimPlugin)
        .add_plugins(LedStripSimPlugin)
        .add_plugins(RobotoFontPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, render_effect_frame)
        .run();
}

fn setup(
    windows: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut laser_materials: ResMut<Assets<LaserSimMaterial>>,
    mut ledstrip_materials: ResMut<Assets<LedStripSimMaterial>>,
) {
    // Assumes we only have one window
    let window = windows.single();

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SimWidgetBundle::new(
        &mut meshes,
        &mut laser_materials,
        window,
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.9),
    ));
    commands.spawn(SimWidgetBundle::new(
        &mut meshes,
        &mut ledstrip_materials,
        window,
        Vec2::new(0.0, 0.925),
        Vec2::new(1.0, 0.05),
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
            TextStyle {
                font: fonts::roboto(),
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        SimulationStateText,
    ));
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut simwidgets: Query<(&mut Transform, &SimWidget)>,
    mut ledstrip_sim_materials: ResMut<Assets<LedStripSimMaterial>>,
    mut laser_sim_materials: ResMut<Assets<LaserSimMaterial>>,
) {
    for e in resize_reader.read() {
        let window_size = Vec2::new(e.width, e.height);

        for (mut transform, simwidget) in simwidgets.iter_mut() {
            (*transform) = simwidget.compute_transform(window_size);
        }

        for (_, ledstrip_sim_material) in ledstrip_sim_materials.iter_mut() {
            ledstrip_sim_material.update_window_size(window_size);
        }

        for (_, laser_sim_material) in laser_sim_materials.iter_mut() {
            laser_sim_material.update_window_size(window_size);
        }
    }
}

fn render_effect_frame(
    time: Res<Time>,
    mut effect_renderer: ResMut<EffectRenderer>,
    mut ledstrip_sim_materials: ResMut<Assets<LedStripSimMaterial>>,
    mut laser_sim_materials: ResMut<Assets<LaserSimMaterial>>,
    mut simulation_state_texts: Query<&mut Text, With<SimulationStateText>>,
) {
    use color::Color;
    let mut framebuffer = vec![color::RGB::zero(); 1024];
    let effect_state = effect_renderer
        .as_mut()
        .render_next_frame(&mut framebuffer, time.as_ref());

    for mut simulation_state_text in simulation_state_texts.iter_mut() {
        simulation_state_text.sections[0].value = effect_state.clone();
    }

    let rgb_colors: Vec<Vec4> = framebuffer
        .into_iter()
        .map(|srgb_col| {
            let linrgb_col = color::palette::LinSrgb::from_color(srgb_col.into_format());
            Vec4::new(linrgb_col.red, linrgb_col.green, linrgb_col.blue, 0.)
        })
        .collect();

    for (_, laser_sim_material) in laser_sim_materials.iter_mut() {
        laser_sim_material.effect_data = rgb_colors.clone();
    }
    for (_, ledstrip_sim_material) in ledstrip_sim_materials.iter_mut() {
        ledstrip_sim_material.effect_data = rgb_colors.clone();
    }
}
