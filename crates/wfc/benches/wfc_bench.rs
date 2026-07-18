use criterion::{Criterion, criterion_group, criterion_main};
use wfc::core::wfc;
use wfc::model::direction::ALL_DIRECTIONS;
use wfc::model::simple_bit_set::SimpleBitSet;

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

fn bench_wfc_8x8(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 8x8 checkerboard", |b| {
        b.iter(|| wfc(8, 8, &rules, &freqs, 2, SEED));
    });
}

fn bench_wfc_16x16(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 16x16 checkerboard", |b| {
        b.iter(|| wfc(16, 16, &rules, &freqs, 2, SEED));
    });
}

fn bench_wfc_32x32(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 32x32 checkerboard", |b| {
        b.iter(|| wfc(32, 32, &rules, &freqs, 2, SEED));
    });
}

criterion_group!(benches, bench_wfc_8x8, bench_wfc_16x16, bench_wfc_32x32);
criterion_main!(benches);
