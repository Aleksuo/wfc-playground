use criterion::{Criterion, criterion_group, criterion_main};
use image::ImageReader;
use wfc::core::{WfcConfig, wfc};
use wfc::preprocessing::create_pattern_model;

const SEED: u64 = 10;

fn preprocess_creatures_image(width: u32, height: u32) -> WfcConfig {
    let image_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../input/creatures.bmp");
    let input_img = ImageReader::open(image_path)
        .expect("Unable to load image")
        .decode()
        .expect("Unable to decode image");
    let pattern_model = create_pattern_model(input_img, 4, 4);
    WfcConfig {
        output_width: width,
        output_height: height,
        num_patterns: pattern_model.patterns.len(),
        adj_rules: pattern_model.adjadency_rules,
        frequency_hints: pattern_model.frequency_hints,
        seed: SEED,
    }
}

fn bench_wfc_creatures_4x4(c: &mut Criterion) {
    let config = preprocess_creatures_image(4, 4);
    c.bench_function("wfc 4x4 creatures", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_creatures_8x8(c: &mut Criterion) {
    let config = preprocess_creatures_image(8, 8);
    c.bench_function("wfc 8x8 creatures", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_creatures_16x16(c: &mut Criterion) {
    let config = preprocess_creatures_image(16, 16);
    c.bench_function("wfc 16x16 creatures", |b| {
        b.iter(|| wfc(&config));
    });
}

criterion_group!(
    creatures_benches,
    bench_wfc_creatures_4x4,
    bench_wfc_creatures_8x8,
    bench_wfc_creatures_16x16
);
criterion_main!(creatures_benches);
