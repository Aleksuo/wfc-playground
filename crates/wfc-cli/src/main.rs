use std::{num::NonZeroU32, process::ExitCode};

use image::ImageReader;

use wfc::{
    core::{ContradictionStrategy, WfcConfig, wfc},
    postprocessing::reconstruct_image,
    preprocessing::create_pattern_model,
};

fn main() -> ExitCode {
    let input_img = ImageReader::open("./input/beach.bmp")
        .expect("Unable to open input image")
        .decode()
        .expect("Unable to decode input image");
    let result = create_pattern_model(input_img, 4, 4);
    let grid_width = 64;
    let grid_height = 64;
    let config = WfcConfig {
        output_width: grid_width,
        output_height: grid_height,
        num_patterns: result.patterns.len(),
        adj_rules: result.adjadency_rules,
        frequency_hints: result.frequency_hints,
        run_seed: 10,
        contradiction_strategy: ContradictionStrategy::Retry {
            max_attempts: NonZeroU32::new(5).unwrap(),
        },
    };
    if let Ok(output) = wfc(&config) {
        let img = reconstruct_image(
            &output,
            grid_width,
            grid_height,
            &result.patterns,
            &result.palette,
            result.pattern_width,
            result.pattern_height,
        );
        std::fs::create_dir_all(".output").expect("Unable to create output directory");
        img.save(".output/output.bmp")
            .expect("Unable to save output");
    } else {
        return ExitCode::from(0);
    }
    ExitCode::from(1)
}
