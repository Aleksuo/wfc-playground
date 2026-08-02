use criterion::{Criterion, criterion_group, criterion_main};
use image::{GenericImageView, ImageReader};
use wfc::core::{ContradictionStrategy, WfcModel, WfcRunConfig, solve};
use wfc::model::{dimensions::Dimensions, sampled::Sampled};
use wfc::preprocessing::create_pattern_model;

const SEED: u64 = 10;

fn preprocess_creatures_image() -> WfcModel {
    let image_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../input/creatures.bmp");
    let input_img = ImageReader::open(image_path)
        .expect("Unable to load image")
        .decode()
        .expect("Unable to decode image");
    let (width, height) = input_img.dimensions();
    let input_dimensions = Dimensions::new([width, height]).expect("Input image is empty");
    let sampled = Sampled::from_fn(input_dimensions, |[x, y]| input_img.get_pixel(x, y));

    let pattern_dimensions = Dimensions::new([4, 4]).expect("4x4 is non-empty");
    let pattern_model = create_pattern_model(&sampled, &pattern_dimensions)
        .expect("Pattern does not fit the input image");
    WfcModel {
        num_patterns: pattern_model.patterns.len(),
        adj_rules: pattern_model.adjadency_rules,
        frequency_hints: pattern_model.frequency_hints,
    }
}

fn run_config(width: u32, height: u32) -> WfcRunConfig {
    WfcRunConfig {
        output_width: width,
        output_height: height,
        seed: SEED,
        contradiction_strategy: ContradictionStrategy::Fail,
    }
}

fn bench_wfc_creatures_4x4(c: &mut Criterion) {
    let model = preprocess_creatures_image();
    let run_config = run_config(4, 4);
    c.bench_function("wfc 4x4 creatures", |b| {
        b.iter(|| solve(&model, &run_config));
    });
}

fn bench_wfc_creatures_8x8(c: &mut Criterion) {
    let model = preprocess_creatures_image();
    let run_config = run_config(8, 8);
    c.bench_function("wfc 8x8 creatures", |b| {
        b.iter(|| solve(&model, &run_config));
    });
}

fn bench_wfc_creatures_16x16(c: &mut Criterion) {
    let model = preprocess_creatures_image();
    let run_config = run_config(16, 16);
    c.bench_function("wfc 16x16 creatures", |b| {
        b.iter(|| solve(&model, &run_config));
    });
}

criterion_group!(
    creatures_benches,
    bench_wfc_creatures_4x4,
    bench_wfc_creatures_8x8,
    bench_wfc_creatures_16x16
);
criterion_main!(creatures_benches);
