use criterion::{Criterion, criterion_group, criterion_main};
use image::ImageReader;
use wfc::core::{WfcConfig, wfc};
use wfc::model::direction::ALL_DIRECTIONS;
use wfc::model::simple_bit_set::SimpleBitSet;
use wfc::preprocessing::create_pattern_model;

const SEED: u64 = 12;

fn checkerboard_rules() -> Vec<SimpleBitSet> {
    let num_patterns = 2;
    let num_directions = ALL_DIRECTIONS.len();
    let mut rules = vec![SimpleBitSet::new(num_patterns); num_patterns * num_directions];
    for dir in ALL_DIRECTIONS {
        // Pattern 0 can be next to pattern 1 in all directions
        rules[dir as usize].set(1);
        // Pattern 1 can be next to pattern 0 in all directions
        rules[num_directions + dir as usize].set(0);
    }
    rules
}

fn checkerboard_frequencies() -> Vec<u32> {
    vec![1, 1]
}

fn bench_wfc_simple_8x8(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    let config = WfcConfig {
        output_width: 8,
        output_height: 8,
        adj_rules: rules,
        frequency_hints: freqs,
        num_patterns: 2,
        seed: SEED,
    };
    c.bench_function("wfc 8x8 checkerboard", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_simple_16x16(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    let config = WfcConfig {
        output_width: 16,
        output_height: 16,
        adj_rules: rules,
        frequency_hints: freqs,
        num_patterns: 2,
        seed: SEED,
    };
    c.bench_function("wfc 16x16 checkerboard", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_simple_32x32(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    let config = WfcConfig {
        output_width: 32,
        output_height: 32,
        adj_rules: rules,
        frequency_hints: freqs,
        num_patterns: 2,
        seed: SEED,
    };
    c.bench_function("wfc 32x32 checkerboard", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_beach_8x8(c: &mut Criterion) {
    let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../input/beach.bmp");
    let input_img = ImageReader::open(image_path)
        .expect("Unable to load image")
        .decode()
        .expect("Unable to decode image");
    let pattern_model = create_pattern_model(input_img, 4, 4);
    let width = 8;
    let height = 8;
    let config = WfcConfig {
        output_width: width,
        output_height: height,
        num_patterns: pattern_model.patterns.len(),
        adj_rules: pattern_model.adjadency_rules,
        frequency_hints: pattern_model.frequency_hints,
        seed: 10,
    };
    c.bench_function("wfc 8x8 beach", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_beach_16x16(c: &mut Criterion) {
    let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../input/beach.bmp");
    let input_img = ImageReader::open(image_path)
        .expect("Unable to load image")
        .decode()
        .expect("Unable to decode image");
    let pattern_model = create_pattern_model(input_img, 4, 4);
    let width = 16;
    let height = 16;
    let config = WfcConfig {
        output_width: width,
        output_height: height,
        num_patterns: pattern_model.patterns.len(),
        adj_rules: pattern_model.adjadency_rules,
        frequency_hints: pattern_model.frequency_hints,
        seed: 10,
    };
    c.bench_function("wfc 16x16 beach", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_beach_32x32(c: &mut Criterion) {
    let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../input/beach.bmp");
    let input_img = ImageReader::open(image_path)
        .expect("Unable to load image")
        .decode()
        .expect("Unable to decode image");
    let pattern_model = create_pattern_model(input_img, 4, 4);
    let width = 32;
    let height = 32;
    let config = WfcConfig {
        output_width: width,
        output_height: height,
        num_patterns: pattern_model.patterns.len(),
        adj_rules: pattern_model.adjadency_rules,
        frequency_hints: pattern_model.frequency_hints,
        seed: 10,
    };
    c.bench_function("wfc 32x32 beach", |b| {
        b.iter(|| wfc(&config));
    });
}

criterion_group!(
    simple_benches,
    bench_wfc_simple_8x8,
    bench_wfc_simple_16x16,
    bench_wfc_simple_32x32
);

criterion_group!(
    beach_benches,
    bench_wfc_beach_8x8,
    bench_wfc_beach_16x16,
    bench_wfc_beach_32x32
);
criterion_main!(simple_benches, beach_benches);
