use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, ItemId, LibHeif};
use pdfium_render::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use image::{
    io::Reader as ImageReader,
    codecs::{
        jpeg::JpegEncoder,
        png::PngEncoder,
    },
    {DynamicImage, ImageBuffer, ImageEncoder, RgbaImage}
};
use webp::Encoder;

pub fn convert_jpeg_to_webp(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let encoder = Encoder::from_image(&img)?;
    let webp_data = encoder.encode_lossless();
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&webp_data)?;
    Ok(())
}

pub fn convert_webp_to_jpeg(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = JpegEncoder::new_with_quality(&mut writer, 80);
    encoder.write_image(img.to_rgb8().as_raw(), img.width(), img.height(), image::ColorType::Rgb8.into())?;
    Ok(())
}

pub fn convert_jpeg_to_png(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = PngEncoder::new(&mut writer);
    encoder.write_image(img.to_rgba8().as_raw(), img.width(), img.height(), image::ColorType::Rgba8.into())?;
    Ok(())
}

pub fn convert_png_to_jpeg(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = JpegEncoder::new_with_quality(&mut writer, 80);
    encoder.write_image(img.to_rgb8().as_raw(), img.width(), img.height(), image::ColorType::Rgb8.into())?;
    Ok(())
}

pub fn convert_png_to_webp(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let encoder = webp::Encoder::from_image(&img)?;
    let webp_data = encoder.encode_lossless();
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&webp_data)?;
    Ok(())
}

pub fn convert_webp_to_png(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let img = match img {
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(img),
        DynamicImage::ImageRgba8(img) => DynamicImage::ImageRgba8(img),
        _ => return Err("Unsupported image format".into()),
    };
    let output_file = File::create(output_file_path)?;
    let writer = BufWriter::new(output_file);
    let encoder = PngEncoder::new(writer);
    encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into())?;
    Ok(())
}

pub fn convert_pdf_to_image(input_path: &str, output_file_path: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bindings = Pdfium::bind_to_system_library()?;
    let library = Pdfium::new(bindings);
    let document = library.load_pdf_from_file(input_path, None)?;

    if document.pages().len() == 0 {
        return Err("No pages found in the PDF document".into());
    }

    let page = document.pages().get(0).unwrap();

    let bitmap = page.render_with_config(
        &PdfRenderConfig::new()
            .set_target_width(800)
            .set_target_height(600),
    )?;

    let img = DynamicImage::ImageRgb8(bitmap.as_image().into());
    
    if img.width() == 0 || img.height() == 0 {
        return Err("Empty image".into());
    }

    let mut output_file = File::create(output_file_path)?;

    match format {
        "jpeg" => {
            let encoder = JpegEncoder::new_with_quality(&mut output_file, 80);
            encoder.write_image(img.to_rgb8().as_raw(), img.width(), img.height(), image::ColorType::Rgb8.into())?;
        },
        "png" => {
            let encoder = PngEncoder::new(&mut output_file);
            encoder.write_image(img.to_rgba8().as_raw(), img.width(), img.height(), image::ColorType::Rgba8.into())?;
        },
        "webp" => {
            let encoder = Encoder::from_image(&img)?;
            let webp_data = encoder.encode_lossless();
            output_file.write_all(&webp_data)?;
        },
        _ => return Err("Unsupported format".into()),
    }

    Ok(())
}

pub fn convert_heic_to_jpeg(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file("./data/test.heif")?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 1652);
    assert_eq!(handle.height(), 1791);

    // Get Exif
    let mut meta_ids: Vec<ItemId> = vec![0; 1];
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    assert_eq!(count, 1);
    let exif: Vec<u8> = handle.metadata(meta_ids[0])?;

    // Decode the image
    let image = lib_heif.decode(
        &handle, 
        ColorSpace::Rgb(RgbChroma::Rgb), 
        None,
    )?;
    assert_eq!(
        image.color_space(), 
        Some(ColorSpace::Rgb(RgbChroma::Rgb)),
    );
    assert_eq!(image.width(), 1652);
    assert_eq!(image.height(), 1791);

    // Scale the image
    let small_img = image.scale(1024, 800, None)?;
    assert_eq!(small_img.width(), 1024);
    assert_eq!(small_img.height(), 800);

    // Get "pixels"
    let planes = small_img.planes();
    let interleaved_plane = planes.interleaved.unwrap();
    assert_eq!(interleaved_plane.width, 1024);
    assert_eq!(interleaved_plane.height, 800);
    assert!(!interleaved_plane.data.is_empty());
    assert!(interleaved_plane.stride > 0);

    Ok(())
}

pub fn convert_heic_to_png(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = HeifContext::read_from_file(input_path)?;
    let handle = ctx.primary_image_handle()?;
    let img = handle.decode(ColorSpace::Rgb(Chroma::Rgb), true)?;
    
    let width = img.width();
    let height = img.height();
    let data = img.to_vec()?;
    
    let image_buffer = ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, data).unwrap();
    let image = DynamicImage::ImageRgb8(image_buffer);

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    let encoder = PngEncoder::new(&mut writer);
    encoder.write_image(image.as_rgba8().unwrap().as_raw(), image.width(), image.height(), image::ColorType::Rgba8.into())?;
    Ok(())
}

pub fn convert_heic_to_webp(input_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = HeifContext::read_from_file(input_path)?;
    let handle = ctx.primary_image_handle()?;
    let img = handle.decode(ColorSpace::Rgb(Chroma::Rgb), true)?;
    
    let width = img.width();
    let height = img.height();
    let data = img.to_vec()?;
    
    let image_buffer = ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, data).unwrap();
    let image = DynamicImage::ImageRgb8(image_buffer);

    let encoder = webp::Encoder::from_image(&image)?;
    let webp_data = encoder.encode_lossless();

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&webp_data)?;
    Ok(())
}