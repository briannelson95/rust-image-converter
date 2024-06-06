mod conversions;

use eframe::egui;
use rfd::FileDialog;
use crate::conversions::convert_jpeg_to_webp;

struct MyApp {
    // counter: i32,
    input_path: String,
    output_path: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // counter: 0,
            input_path: String::new(),
            output_path: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Hello, world!");
            // ui.label(format!("Counter: {}", self.counter));
            // if ui.button("Increment").clicked() {
            //     self.counter += 1;
            // }

            // ui.separator();

            // Input Path Section
            ui.horizontal(|ui| {
                ui.label("Input Path:");
                ui.text_edit_singleline(&mut self.input_path);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Image files", &["jpg", "jpeg", "png", "bmp", "gif", "tiff"])
                        .pick_file()
                    {
                        self.input_path = path.display().to_string();
                    }
                }
            });

            ui.separator();

            // Output Path Section
            ui.horizontal(|ui| {
                ui.label("Output Path:");
                ui.text_edit_singleline(&mut self.output_path);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.output_path = path.display().to_string();
                    }
                }
            });

            ui.separator();

            // Convert Button
            if ui.button("Convert").clicked() {
                if let Err(e) = convert_jpeg_to_webp(&self.input_path, &self.output_path) {
                    ui.label(format!("Error: {}", e));
                } else {
                    ui.label("Conversion successful!");
                }
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Image Converter",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
