use criterion::{Criterion, criterion_group, criterion_main};
use wfc::core::{WfcConfig, wfc};
use wfc::model::direction::ALL_DIRECTIONS;
use wfc::model::simple_bit_set::SimpleBitSet;

const SEED: u64 = 12;

fn preprocess_checkerboard(width: u32, height: u32) -> WfcConfig {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    WfcConfig {
        output_width: width,
        output_height: height,
        adj_rules: rules,
        frequency_hints: freqs,
        num_patterns: 2,
        seed: SEED,
    }
}

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
    let config = preprocess_checkerboard(8, 8);
    c.bench_function("wfc 8x8 checkerboard", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_simple_16x16(c: &mut Criterion) {
    let config = preprocess_checkerboard(16, 16);
    c.bench_function("wfc 16x16 checkerboard", |b| {
        b.iter(|| wfc(&config));
    });
}

fn bench_wfc_simple_32x32(c: &mut Criterion) {
    let config = preprocess_checkerboard(32, 32);
    c.bench_function("wfc 32x32 checkerboard", |b| {
        b.iter(|| wfc(&config));
    });
}

criterion_group!(
    simple_benches,
    bench_wfc_simple_8x8,
    bench_wfc_simple_16x16,
    bench_wfc_simple_32x32
);
criterion_main!(simple_benches);
