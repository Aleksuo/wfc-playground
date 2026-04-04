use std::collections::{HashMap, HashSet};

use criterion::{Criterion, criterion_group, criterion_main};
use wfc::core::wfc;
use wfc::model::direction::Direction;

fn checkerboard_rules() -> HashMap<(u16, Direction), HashSet<u16>> {
    let mut rules = HashMap::new();
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        rules.insert((0, dir), HashSet::from([1]));
        rules.insert((1, dir), HashSet::from([0]));
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
        b.iter(|| wfc(8, 8, &rules, &freqs, 1));
    });
}

fn bench_wfc_16x16(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 16x16 checkerboard", |b| {
        b.iter(|| wfc(16, 16, &rules, &freqs, 1));
    });
}

fn bench_wfc_32x32(c: &mut Criterion) {
    let rules = checkerboard_rules();
    let freqs = checkerboard_frequencies();
    c.bench_function("wfc 32x32 checkerboard", |b| {
        b.iter(|| wfc(32, 32, &rules, &freqs, 1));
    });
}

criterion_group!(benches, bench_wfc_8x8, bench_wfc_16x16, bench_wfc_32x32);
criterion_main!(benches);
