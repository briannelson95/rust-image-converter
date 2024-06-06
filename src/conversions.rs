use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use webp::Encoder;
use image::DynamicImage;

pub fn convert_jpeg_to_webp(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input image
    let img = ImageReader::open(input_path)?.decode()?;
    
    // Convert the image to WebP format
    let encoder = Encoder::from_image(&img)?;
    let webp_data = encoder.encode_lossless();
    
    // Create the output file path
    let input_filename = Path::new(input_path).file_stem().unwrap().to_str().unwrap();
    let output_filepath = format!("{}/{}.webp", output_path, input_filename);
    
    // Write the WebP data to the output file
    let output_file = File::create(output_filepath)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&webp_data)?;

    Ok(())
}

pub fn convert_webp_to_jpeg(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let input_filename = Path::new(input_path).file_stem().unwrap().to_str().unwrap();
    let output_filepath = format!("{}/{}.jpg", output_path, input_filename);
    let output_file = File::create(output_filepath)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = JpegEncoder::new_with_quality(&mut writer, 80);
    encoder.write_image(img.to_rgb8().as_raw(), img.width(), img.height(), image::ColorType::Rgb8.into())?;
    Ok(())
}

pub fn convert_jpeg_to_png(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let input_filename = Path::new(input_path).file_stem().unwrap().to_str().unwrap();
    let output_filepath = format!("{}/{}.png", output_path, input_filename);
    let output_file = File::create(output_filepath)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = PngEncoder::new(&mut writer);
    encoder.write_image(img.to_rgba8().as_raw(), img.width(), img.height(), image::ColorType::Rgba8.into())?;
    Ok(())
}

pub fn convert_png_to_jpeg(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let input_filename = Path::new(input_path).file_stem().unwrap().to_str().unwrap();
    let output_filepath = format!("{}/{}.jpg", output_path, input_filename);
    let output_file = File::create(output_filepath)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = JpegEncoder::new_with_quality(&mut writer, 80);
    encoder.write_image(img.to_rgb8().as_raw(), img.width(), img.height(), image::ColorType::Rgb8.into())?;
    Ok(())
}

pub fn convert_png_to_webp(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let encoder = webp::Encoder::from_image(&img)?;
    let webp_data = encoder.encode_lossless();
    let input_filename = Path::new(input_path).file_stem().unwrap().to_str().unwrap();
    let output_filepath = format!("{}/{}.webp", output_path, input_filename);
    let output_file = File::create(output_filepath)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&webp_data)?;
    Ok(())
}

pub fn convert_webp_to_png(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input image
    let img = ImageReader::open(input_path)?.decode()?;
    
    // Convert the image to PNG format
    let img = match img {
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(img),
        DynamicImage::ImageRgba8(img) => DynamicImage::ImageRgba8(img),
        _ => return Err("Unsupported image format".into()),
    };
    
    // Create the output file path
    let input_filename = Path::new(input_path).file_stem().unwrap().to_str().unwrap();
    let output_filepath = format!("{}/{}.png", output_path, input_filename);
    
    // Write the PNG data to the output file
    let output_file = File::create(output_filepath)?;
    let writer = BufWriter::new(output_file);
    let encoder = PngEncoder::new(writer);
    encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into())?;

    Ok(())
}