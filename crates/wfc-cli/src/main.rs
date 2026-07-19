use image::ImageReader;

use wfc::{
    core::{WfcConfig, wfc},
    postprocessing::reconstruct_image,
    preprocessing::create_pattern_model,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_img = ImageReader::open("./input/beach.bmp")?.decode()?;
    let result = create_pattern_model(input_img, 4, 4);
    let grid_width = 64;
    let grid_height = 64;
    let config = WfcConfig {
        output_width: grid_width,
        output_height: grid_height,
        num_patterns: result.patterns.len(),
        adj_rules: result.adjadency_rules,
        frequency_hints: result.frequency_hints,
        seed: 10,
    };
    let output = wfc(&config);
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
