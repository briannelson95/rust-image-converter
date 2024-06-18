mod conversions;

use eframe::egui;
use rfd::FileDialog;
use std::{
    path::{Path, PathBuf},
    fs::File,
};
use crate::conversions::{
    convert_jpeg_to_webp, 
    convert_webp_to_jpeg, 
    convert_jpeg_to_png,
    convert_png_to_jpeg,
    convert_png_to_webp,
    convert_webp_to_png,
    convert_pdf_to_image,
    convert_heic_to_jpeg,
    convert_heic_to_png,
    convert_heic_to_webp,
};
use open;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ConversionType {
    JpegToWebp,
    WebpToJpeg,
    WebpToPng,
    JpegToPng,
    PngToJpeg,
    PngToWebp,
    PdfToJpeg,
    PdfToPng,
    PdfToWebp,
    HeicToJpeg, // New conversion type
    HeicToPng,  // New conversion type
    HeicToWebp, // New conversion type
}

struct MyApp {
    input_path: String,
    output_path: String,
    file_name: String,
    conversion_type: Option<ConversionType>,
    available_conversions: Vec<ConversionType>,
    open_folder_after_conversion: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            file_name: String::new(),
            conversion_type: None,
            available_conversions: Vec::new(),
            open_folder_after_conversion: true,
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
                "pdf" => vec![ConversionType::PdfToJpeg, ConversionType::PdfToPng, ConversionType:: PdfToWebp],
                "heic" => vec![ConversionType::HeicToJpeg, ConversionType::HeicToPng, ConversionType::HeicToWebp],
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
                ui.label("Choose an image:");
                ui.text_edit_singleline(&mut self.input_path);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Image files", &["jpg", "jpeg", "webp", "png", "pdf"])
                        .pick_file()
                    {
                        self.input_path = path.display().to_string();
                        self.available_conversions();
                    }
                }
                ui.label("Conversion Type:");
                egui::ComboBox::from_label("")
                    .selected_text(match self.conversion_type {
                        Some(ConversionType::JpegToWebp) => "JPEG to WebP",
                        Some(ConversionType::JpegToPng) => "JPEG to PNG",
                        Some(ConversionType::WebpToJpeg) => "WebP to JPEG",
                        Some(ConversionType::WebpToPng) => "WebP to PNG",
                        Some(ConversionType::PngToJpeg) => "PNG to JPEG",
                        Some(ConversionType::PngToWebp) => "PNG to WebP",
                        Some(ConversionType::PdfToJpeg) => "PDF to JPEG",
                        Some(ConversionType::PdfToPng) => "PDF to PNG",
                        Some(ConversionType::PdfToWebp) => "PDF to WebP",
                        Some(ConversionType::HeicToJpeg) => "jpeg",
                        Some(ConversionType::HeicToPng) => "png",
                        Some(ConversionType::HeicToWebp) => "webp",
                        None => "Select Conversion",
                    })
                    .show_ui(ui, |ui| {
                        for conversion in &self.available_conversions {
                            ui.selectable_value(&mut self.conversion_type, Some(*conversion), match conversion {
                                ConversionType::JpegToWebp => "JPEG to WebP",
                                ConversionType::JpegToPng => "JPEG to PNG",
                                ConversionType::WebpToJpeg => "WebP to JPEG",
                                ConversionType::WebpToPng => "Webp to PNG",
                                ConversionType::PngToJpeg => "PNG to JPEG",
                                ConversionType::PngToWebp => "PNG to WebP",
                                ConversionType::PdfToJpeg => "PDF to JPEG",
                                ConversionType::PdfToPng => "PDF to PNG",
                                ConversionType::PdfToWebp => "PDF to WebP",
                                ConversionType::HeicToJpeg => "HEIC to JPEG",
                                ConversionType::HeicToPng => "HEIC to PNG",
                                ConversionType::HeicToWebp => "HEIC to WebP",
                            });
                        }
                    });
            });

            ui.separator();

            // Output Path Section
            ui.horizontal(|ui| {
                ui.label("Save to:");
                ui.text_edit_singleline(&mut self.output_path);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.output_path = path.display().to_string();
                    }
                }
                ui.label("File name:");
                ui.text_edit_singleline(&mut self.file_name);
            });

            ui.separator();

            // Convert Button
            ui.horizontal(|ui| {
                if ui.button("Convert").clicked() {
                    if self.output_path.is_empty() {
                        self.output_path = Path::new(&self.input_path)
                            .parent()
                            .unwrap()
                            .display()
                            .to_string();
                    }
    
                    let output_path = PathBuf::from(&self.output_path);
                    let output_file_name = if self.file_name.is_empty() {
                        Path::new(&self.input_path)
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    } else {
                        self.file_name.clone()
                    };
    
                    let output_extension = match self.conversion_type {
                        Some(ConversionType::JpegToWebp) => "webp",
                        Some(ConversionType::WebpToJpeg) => "jpeg",
                        Some(ConversionType::JpegToPng) => "png",
                        Some(ConversionType::PngToJpeg) => "jpeg",
                        Some(ConversionType::PngToWebp) => "webp",
                        Some(ConversionType::WebpToPng) => "png",
                        Some(ConversionType::PdfToJpeg) => "jpeg",
                        Some(ConversionType::PdfToPng) => "png",
                        Some(ConversionType::PdfToWebp) => "webp",
                        Some(ConversionType::HeicToJpeg) => "jpeg",
                        Some(ConversionType::HeicToPng) => "png",
                        Some(ConversionType::HeicToWebp) => "webp",
                        _ => "",
                    };
    
                    let final_output_path = output_path.join(format!("{}.{}", output_file_name, output_extension));
    
                    if let Err(e) = File::create(&final_output_path) {
                        ui.label(format!("Error creating file: {}", e));
                        return;
                    }
    
                    let result = match self.conversion_type {
                        Some(ConversionType::JpegToWebp) => convert_jpeg_to_webp(&self.input_path, final_output_path.to_str().unwrap()),
                        Some(ConversionType::WebpToJpeg) => convert_webp_to_jpeg(&self.input_path, final_output_path.to_str().unwrap()),
                        Some(ConversionType::JpegToPng) => convert_jpeg_to_png(&self.input_path, final_output_path.to_str().unwrap()),
                        Some(ConversionType::PngToJpeg) => convert_png_to_jpeg(&self.input_path, final_output_path.to_str().unwrap()),
                        Some(ConversionType::PngToWebp) => convert_png_to_webp(&self.input_path, final_output_path.to_str().unwrap()),
                        Some(ConversionType::WebpToPng) => convert_webp_to_png(&self.input_path, final_output_path.to_str().unwrap()),
                        Some(ConversionType::PdfToJpeg) => convert_pdf_to_image(&self.input_path, final_output_path.to_str().unwrap(), "jpeg"),
                        Some(ConversionType::PdfToPng) => convert_pdf_to_image(&self.input_path, final_output_path.to_str().unwrap(), "png"),
                        Some(ConversionType::PdfToWebp) => convert_pdf_to_image(&self.input_path, final_output_path.to_str().unwrap(), "webp"),
                        Some(ConversionType::HeicToJpeg) => convert_heic_to_jpeg(&self.input_path, final_output_path.to_str().unwrap()), // New handler
                        Some(ConversionType::HeicToPng) => convert_heic_to_png(&self.input_path, final_output_path.to_str().unwrap()),   // New handler
                        Some(ConversionType::HeicToWebp) => convert_heic_to_webp(&self.input_path, final_output_path.to_str().unwrap()), // New handler
                        _ => Err("Unsupported conversion type".into()),
                    };
    
                    if let Err(e) = result {
                        ui.label(format!("Error during conversion: {}", e));
                    } else {
                        ui.label("Conversion successful!");
                        if self.open_folder_after_conversion {
                            if let Err(e) = open::that(&self.output_path) {
                                ui.label(format!("Failed to open folder: {}", e));
                            }
                        }
                    }
                }
                ui.checkbox(&mut self.open_folder_after_conversion, "Open folder after conversion");
            })
            
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
