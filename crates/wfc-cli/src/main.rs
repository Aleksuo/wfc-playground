use std::{num::NonZeroU32, process::ExitCode};

use image::{GenericImageView, ImageReader};

use wfc::{
    ContradictionStrategy, Dimensions, Sampled, SolverRunConfiguration, create_pattern_model,
    reconstruct_image, solve,
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
    let seed = 10;
    let output_dimensions = Dimensions::new([grid_width, grid_height]).unwrap();
    let compiled_model = rule_model
        .compile()
        .expect("Derived rule model failed validation");
    let run_config = SolverRunConfiguration {
        output_dimensions,
        seed,
        contradiction_strategy: ContradictionStrategy::Retry {
            max_attempts: NonZeroU32::new(5).unwrap(),
        },
    };
    if let Ok(solution) = solve(&compiled_model, &run_config) {
        let img = reconstruct_image(
            &solution.output,
            grid_width,
            grid_height,
            &rule_model.patterns,
            sampled.palette(),
            pattern_dimensions.get_axis(0),
            pattern_dimensions.get_axis(1),
        );
        std::fs::create_dir_all(".output").expect("Unable to create output directory");
        img.save(".output/output.bmp")
            .expect("Unable to save output");
    } else {
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}
