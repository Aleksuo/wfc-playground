use std::{num::NonZeroU32, process::ExitCode};

use image::{GenericImageView, ImageReader};

use wfc::{
    CompiledModel, ContradictionStrategy, Dimensions, Sampled, SolverRunConfiguration,
    create_pattern_model, reconstruct_image, solve,
};

fn main() -> ExitCode {
    let input_img = ImageReader::open("./input/beach.bmp")
        .expect("Unable to open input image")
        .decode()
        .expect("Unable to decode input image");

    let (width, height) = input_img.dimensions();
    let input_dims = Dimensions::new([width, height]).expect("Input image is empty");
    let sampled = Sampled::from_fn(input_dims, |[x, y]| input_img.get_pixel(x, y));

    let pattern_dimensions = Dimensions::new([4, 4]).unwrap();
    let rule_model = create_pattern_model(&sampled, &pattern_dimensions)
        .expect("Pattern does not fit the input image");

    let grid_width = 64;
    let grid_height = 64;
    let model = CompiledModel {
        num_patterns: rule_model.patterns.len(),
        adj_rules: rule_model.adjadency_rules,
        frequency_hints: rule_model.frequency_hints,
    };
    let run_config = SolverRunConfiguration {
        output_width: grid_width,
        output_height: grid_height,
        seed: 10,
        contradiction_strategy: ContradictionStrategy::Retry {
            max_attempts: NonZeroU32::new(5).unwrap(),
        },
    };
    if let Ok(output) = solve(&model, &run_config) {
        let img = reconstruct_image(
            &output,
            grid_width,
            grid_height,
            &rule_model.patterns,
            sampled.palette(),
            pattern_dimensions.get(0),
            pattern_dimensions.get(1),
        );
        std::fs::create_dir_all(".output").expect("Unable to create output directory");
        img.save(".output/output.bmp")
            .expect("Unable to save output");
    } else {
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}
