use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;  // Import the Write trait
use image::io::Reader as ImageReader;
use webp::Encoder;

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
