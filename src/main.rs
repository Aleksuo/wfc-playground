use image::ImageReader;

use crate::{postprocessing::reconstruct_image, preprocessing::overlap_model};

mod core;
mod model;
mod postprocessing;
mod preprocessing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_img = ImageReader::open("./input/beach.bmp")?.decode()?;
    let (palette, adjadency_rules, frequency_hints) = overlap_model(input_img);
    let output_width = 64;
    let output_height = 64;
    let max_val = (palette.len() - 1) as u16;
    let output = crate::core::wfc(
        output_width,
        output_height,
        &adjadency_rules,
        &frequency_hints,
        max_val,
    );
    let img = reconstruct_image(&output, output_width, output_height, &palette);
    std::fs::create_dir_all(".output")?;
    img.save(".output/output.bmp")?;
    Ok(())
}
