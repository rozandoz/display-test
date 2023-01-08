#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::Pos2;
use eframe::{egui, Theme};
use egui::{containers::*, *};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        default_theme: Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "Display test",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

struct App {
    position: Pos2,
    last_update_time: f64,
    animation_enabled: bool,
    antialiasing_enabled: bool,
    display_horizontal: bool,
    display_vertical: bool,
    speeds: Vec<u32>,
    speed: u32,
}

impl Default for App {
    fn default() -> Self {
        let speeds = vec![20, 30, 40, 50, 60, 90, 120, 150];

        Self {
            position: Pos2::new(0.0, 0.0),
            last_update_time: 0.0,
            animation_enabled: true,
            antialiasing_enabled: false,
            display_horizontal: true,
            display_vertical: false,
            speed: speeds[0],
            speeds: speeds,
        }
    }
}

impl App {
    fn main_ui(&mut self, ui: &mut egui::Ui) {
        let color = if ui.visuals().dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        };

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();

            let time = ui.input().time;

            let size = ui.available_size();
            let (_, rect) = ui.allocate_space(size);

            let to_screen = emath::RectTransform::from_to(
                Rect::from_x_y_ranges(0.0..=size.x, 0.0..=size.y),
                rect,
            );

            let thickness = 1.0;
            let mut shapes = vec![];

            {
                let mut x = self.position.x;
                let mut y = self.position.y;

                let dt = time - self.last_update_time;

                if dt > 1.0 / self.speed as f64 {
                    x = (x + 1.0) % size.x;
                    y = (y + 1.0) % size.y;
                    self.last_update_time = time;
                }

                if self.animation_enabled {
                    self.position = Pos2::new(x, y);
                }

                if self.display_horizontal {
                    let mut points = vec![];
                    points.push(to_screen * Pos2::new(0.0, self.position.y));
                    points.push(to_screen * Pos2::new(size.x, self.position.y));

                    shapes.push(epaint::Shape::line(points, Stroke::new(thickness, color)));
                }

                if self.display_vertical {
                    let mut points = vec![];
                    points.push(to_screen * Pos2::new(self.position.x, 0.0));
                    points.push(to_screen * Pos2::new(self.position.x, size.y));

                    shapes.push(epaint::Shape::line(points, Stroke::new(thickness, color)));
                }
            }

            ui.painter().extend(shapes);
        });
    }

    pub fn toggle_antialising(&mut self, ctx: &egui::Context, enabled: bool) {
        let mut to = ctx.tessellation_options();
        to.feathering = enabled;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.toggle_antialising(ctx, self.antialiasing_enabled);

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            global_dark_light_mode_buttons(ui);
            ui.checkbox(&mut self.antialiasing_enabled, "Anti-aliasing");

            ui.separator();

            ui.checkbox(&mut self.animation_enabled, "Animation");
            ui.checkbox(&mut self.display_horizontal, "Horizontal");
            ui.checkbox(&mut self.display_vertical, "Vertical");

            ui.separator();

            ui.label(format!("x:{0} y:{1}", self.position.x, self.position.y));

            egui::ComboBox::from_label("Speed")
                .selected_text(format!("{:?} px/s", self.speed))
                .show_ui(ui, |ui| {
                    for speed in self.speeds.iter() {
                        ui.selectable_value(&mut self.speed, *speed, format!("{:?} px/s", speed));
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_ui(ui);
        });

        ctx.request_repaint();
    }
}
