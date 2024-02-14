use three_d::*;

mod visualizations;

pub fn main() {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();

    let window = Window::new(WindowSettings {
        title: "1D Effects Simulator".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut gui = three_d::GUI::new(&context);

    // TODO remove and move to struct
    let mut viewport_zoom = 1.0;
    let mut scissor_zoom = 1.0;

    let mut laser_sim_widget = visualizations::SimWidget::new(&context, (0.5, 0.45), (0.98, 0.88));
    let mut led_strip_widget = visualizations::SimWidget::new(&context, (0.5, 0.95), (0.98, 0.05));

    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut viewport_zoom, 0.01..=1.0).text("Viewport"));
                    ui.add(Slider::new(&mut scissor_zoom, 0.01..=1.0).text("Scissor"));
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };

        laser_sim_widget.update(viewport);
        led_strip_widget.update(viewport);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 0.0, 1.0, 1.0, 1.0))
            .clear_partially(viewport.into(), ClearState::color(0.1, 0.1, 0.1, 1.0))
            .render_partially(
                viewport.into(),
                &Camera::new_2d(viewport),
                &[laser_sim_widget.obj(), led_strip_widget.obj()],
                &[],
            )
            .write(|| gui.render());

        // Returns default frame output to end the frame
        FrameOutput::default()
    });
}
