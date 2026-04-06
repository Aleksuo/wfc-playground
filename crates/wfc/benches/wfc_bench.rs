use std::collections::HashMap;

use criterion::{Criterion, criterion_group, criterion_main};
use wfc::core::wfc;
use wfc::model::direction::Direction;
use wfc::model::simple_bit_set::SimpleBitSet;

fn checkerboard_rules() -> HashMap<(u16, Direction), SimpleBitSet> {
    let mut rules = HashMap::new();
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut simple_bit_set_1 = SimpleBitSet::new(2);
        simple_bit_set_1.set(0);
        let mut simple_bit_set_2 = SimpleBitSet::new(2);
        simple_bit_set_2.set(1);
        rules.insert((0, dir), simple_bit_set_2);
        rules.insert((1, dir), simple_bit_set_1);
    }
    rules
}

fn checkerboard_frequencies() -> HashMap<u16, u32> {
    HashMap::from([(0, 1), (1, 1)])
}

fn bench_wfc_8x8(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 8x8 checkerboard", |b| {
        b.iter(|| wfc(8, 8, &rules, &freqs, 2));
    });
}

fn bench_wfc_16x16(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 16x16 checkerboard", |b| {
        b.iter(|| wfc(16, 16, &rules, &freqs, 2));
    });
}

fn bench_wfc_32x32(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 32x32 checkerboard", |b| {
        b.iter(|| wfc(32, 32, &rules, &freqs, 2));
    });
}

criterion_group!(benches, bench_wfc_8x8, bench_wfc_16x16, bench_wfc_32x32);
criterion_main!(benches);
