use image::ImageReader;

use crate::{postprocessing::reconstruct_image, preprocessing::create_pattern_model};

mod core;
mod model;
mod postprocessing;
mod preprocessing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_img = ImageReader::open("./input/beach.bmp")?.decode()?;
    let result = create_pattern_model(input_img, 4, 4);
    let grid_width = 64;
    let grid_height = 64;
    let max_val = (result.patterns.len() - 1) as u16;
    let output = crate::core::wfc(
        grid_width,
        grid_height,
        &result.adjadency_rules,
        &result.frequency_hints,
        max_val,
    );
    let img = reconstruct_image(
        &output,
        grid_width,
        grid_height,
        &result.patterns,
        &result.palette,
        result.pattern_width,
        result.pattern_height,
    );
    std::fs::create_dir_all(".output")?;
    img.save(".output/output.bmp")?;
    Ok(())
}
