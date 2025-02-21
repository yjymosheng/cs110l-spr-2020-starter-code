use grid::Grid;
use nannou::prelude::*;
use nannou_egui::{Egui, egui};
mod grid;

struct Model {
    elements: Grid,
    setting: Setting,
    egui: nannou_egui::Egui,
}

struct Setting {
    number_row: usize,
    number_col: usize,
    initial_number: usize,
    speed: usize,
}

fn raw_window_event(app: &App, modle: &mut Model, event: &nannou::winit::event::WindowEvent) {
    modle.egui.handle_raw_event(event);
}
fn model(app: &App) -> Model {
    let elements = Grid::new(5, 5);
    let window = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window).unwrap();
    let egui = Egui::from_window(&window);
    let setting = Setting {
        number_row: 5,
        number_col: 5,
        initial_number: 5,
        speed: 1,
    };
    Model {
        elements,
        setting,
        egui,
    }
}

fn update(app: &App, modle: &mut Model, update: Update) {
    let egui = &mut modle.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Setting").show(&ctx, |ui| {
        ui.label("Initial number");
        let count = modle.setting.number_col * modle.setting.number_row;
        ui.add(egui::Slider::new(
            &mut modle.setting.initial_number,
            1..=count,
        ));

        ui.label("number of row");
        ui.add(egui::Slider::new(
            &mut modle.setting.number_row,
            5..=20,
        ));
        ui.label("number of col");
        ui.add(egui::Slider::new(
            &mut modle.setting.number_col,
            5..=20,
        ));
        ui.label("speed");
        ui.add(egui::Slider::new(
            &mut modle.setting.speed,
            1..=10,
        ));
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Use app time to progress through a sine wave
    // frame.clear(BLACK);
    let draw = app.draw();
    draw.background().color(WHITE);

    let mouse = &app.mouse;

    draw.ellipse()
        .caps_round()
        .xy(mouse.position())
        .w(map_range(mouse.x.sin(), -1.0, 1.0, 40.0, 200.0))
        .h(map_range(mouse.x.sin(), -1.0, 1.0, 40.0, 200.0))
        .rgb(
            random_range(0.0, 255.0),
            random_range(0.0, 255.0),
            random_range(0.0, 255.0),
        );

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
