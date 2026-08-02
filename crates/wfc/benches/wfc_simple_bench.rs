use criterion::{Criterion, criterion_group, criterion_main};
use wfc::core::{ContradictionStrategy, WfcModel, WfcRunConfig, solve};
use wfc::model::direction::ALL_DIRECTIONS;
use wfc::model::rule_model::FrequencyHints;
use wfc::model::simple_bit_set::SimpleBitSet;

const SEED: u64 = 12;

fn preprocess_checkerboard() -> WfcModel {
    let rules = checkerboard_rules();
    let freqs = FrequencyHints::new(checkerboard_frequencies());
    WfcModel {
        adj_rules: rules,
        frequency_hints: freqs,
        num_patterns: 2,
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
    let model = preprocess_checkerboard();
    let run_config = run_config(8, 8);
    c.bench_function("wfc 8x8 checkerboard", |b| {
        b.iter(|| solve(&model, &run_config));
    });
}

fn bench_wfc_simple_16x16(c: &mut Criterion) {
    let model = preprocess_checkerboard();
    let run_config = run_config(16, 16);
    c.bench_function("wfc 16x16 checkerboard", |b| {
        b.iter(|| solve(&model, &run_config));
    });
}

fn bench_wfc_simple_32x32(c: &mut Criterion) {
    let model = preprocess_checkerboard();
    let run_config = run_config(32, 32);
    c.bench_function("wfc 32x32 checkerboard", |b| {
        b.iter(|| solve(&model, &run_config));
    });
}

criterion_group!(
    simple_benches,
    bench_wfc_simple_8x8,
    bench_wfc_simple_16x16,
    bench_wfc_simple_32x32
);
criterion_main!(simple_benches);
