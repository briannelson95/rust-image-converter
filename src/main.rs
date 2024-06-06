mod conversions;

use eframe::egui;
use rfd::FileDialog;
use std::path::Path;
use crate::conversions::
    {
        convert_jpeg_to_webp, 
        convert_webp_to_jpeg, 
        convert_jpeg_to_png,
        convert_png_to_jpeg,
        convert_png_to_webp,
        convert_webp_to_png
    };

#[derive(Clone, Copy, PartialEq, Debug)]
enum ConversionType {
    JpegToWebp,
    WebpToJpeg,
    WebpToPng,
    JpegToPng,
    PngToJpeg,
    PngToWebp,
}

struct MyApp {
    input_path: String,
    output_path: String,
    conversion_type: Option<ConversionType>,
    available_conversions: Vec<ConversionType>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            conversion_type: None,
            available_conversions: Vec::new(),
        }
    }
}

impl MyApp {
    fn available_conversions(&mut self) {
        let path = Path::new(&self.input_path);
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            self.available_conversions = match ext.as_str() {
                "jpg" | "jpeg" => vec![ConversionType::JpegToWebp, ConversionType::JpegToPng],
                "webp" => vec![ConversionType::WebpToJpeg, ConversionType::WebpToPng],
                "png" => vec![ConversionType::PngToJpeg, ConversionType::PngToWebp],
                _ => vec![],
            };
            if !self.available_conversions.is_empty() {
                self.conversion_type = Some(self.available_conversions[0]);
            } else {
                self.conversion_type = None;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Input Path Section
            ui.horizontal(|ui| {
                ui.label("Input Path:");
                ui.text_edit_singleline(&mut self.input_path);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Image files", &["jpg", "jpeg", "webp", "png"])
                        .pick_file()
                    {
                        self.input_path = path.display().to_string();
                        self.available_conversions();
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

            // Conversion Type Dropdown
            if !self.available_conversions.is_empty() {
                egui::ComboBox::from_label("Conversion Type")
                    .selected_text(format!("{:?}", self.conversion_type.unwrap()))
                    .show_ui(ui, |ui| {
                        for conversion in &self.available_conversions {
                            ui.selectable_value(
                                &mut self.conversion_type,
                                Some(*conversion),
                                format!("{:?}", conversion),
                            );
                        }
                    });
            }

            ui.separator();

            // Convert Button
            if ui.button("Convert").clicked() {
                if self.output_path.is_empty() {
                    self.output_path = Path::new(&self.input_path)
                        .parent()
                        .unwrap()
                        .display()
                        .to_string();
                }

                let result = match self.conversion_type {
                    Some(ConversionType::JpegToWebp) => convert_jpeg_to_webp(&self.input_path, &self.output_path),
                    Some(ConversionType::WebpToJpeg) => convert_webp_to_jpeg(&self.input_path, &self.output_path),
                    Some(ConversionType::JpegToPng) => convert_jpeg_to_png(&self.input_path, &self.output_path),
                    Some(ConversionType::PngToJpeg) => convert_png_to_jpeg(&self.input_path, &self.output_path),
                    Some(ConversionType::PngToWebp) => convert_png_to_webp(&self.input_path, &self.output_path),
                    Some(ConversionType::WebpToPng) => convert_webp_to_png(&self.input_path, &self.output_path),
                    _ => Err("Unsupported conversion type".into()),
                };

                if let Err(e) = result {
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
