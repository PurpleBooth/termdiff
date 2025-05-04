use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Cursor;
use termdiff::{diff, ArrowsTheme, DrawDiff, SignsTheme};

fn benchmark_diff(c: &mut Criterion) {
    // Small inputs
    let small_old = "The quick brown fox";
    let small_new = "The quick red fox";

    // Medium inputs
    let medium_old = "The quick brown fox jumps over the lazy dog.\nThis is a second line.\nAnd a third line for testing.";
    let medium_new = "The quick red fox jumps over the sleepy dog.\nThis is a modified second line.\nAnd a third line for testing.";

    // Large inputs (multiple paragraphs)
    let large_old = include_str!("fixtures/large_text_old.txt");
    let large_new = include_str!("fixtures/large_text_new.txt");

    // Benchmark DrawDiff with different themes and input sizes
    let mut group = c.benchmark_group("DrawDiff");

    // Small inputs
    group.bench_function("small_arrows", |b| {
        b.iter(|| {
            let theme = ArrowsTheme::default();
            black_box(format!(
                "{}",
                DrawDiff::new(black_box(small_old), black_box(small_new), &theme)
            ))
        })
    });

    group.bench_function("small_signs", |b| {
        b.iter(|| {
            let theme = SignsTheme::default();
            black_box(format!(
                "{}",
                DrawDiff::new(black_box(small_old), black_box(small_new), &theme)
            ))
        })
    });

    // Medium inputs
    group.bench_function("medium_arrows", |b| {
        b.iter(|| {
            let theme = ArrowsTheme::default();
            black_box(format!(
                "{}",
                DrawDiff::new(black_box(medium_old), black_box(medium_new), &theme)
            ))
        })
    });

    group.bench_function("medium_signs", |b| {
        b.iter(|| {
            let theme = SignsTheme::default();
            black_box(format!(
                "{}",
                DrawDiff::new(black_box(medium_old), black_box(medium_new), &theme)
            ))
        })
    });

    // Large inputs
    group.bench_function("large_arrows", |b| {
        b.iter(|| {
            let theme = ArrowsTheme::default();
            black_box(format!(
                "{}",
                DrawDiff::new(black_box(large_old), black_box(large_new), &theme)
            ))
        })
    });

    group.bench_function("large_signs", |b| {
        b.iter(|| {
            let theme = SignsTheme::default();
            black_box(format!(
                "{}",
                DrawDiff::new(black_box(large_old), black_box(large_new), &theme)
            ))
        })
    });

    group.finish();

    // Benchmark diff function with different themes and input sizes
    let mut group = c.benchmark_group("diff_function");

    // Small inputs
    group.bench_function("small_arrows", |b| {
        b.iter(|| {
            let mut buffer = Cursor::new(Vec::new());
            let theme = ArrowsTheme::default();
            black_box(diff(
                &mut buffer,
                black_box(small_old),
                black_box(small_new),
                &theme,
            ))
            .unwrap();
        })
    });

    group.bench_function("small_signs", |b| {
        b.iter(|| {
            let mut buffer = Cursor::new(Vec::new());
            let theme = SignsTheme::default();
            black_box(diff(
                &mut buffer,
                black_box(small_old),
                black_box(small_new),
                &theme,
            ))
            .unwrap();
        })
    });

    // Medium inputs
    group.bench_function("medium_arrows", |b| {
        b.iter(|| {
            let mut buffer = Cursor::new(Vec::new());
            let theme = ArrowsTheme::default();
            black_box(diff(
                &mut buffer,
                black_box(medium_old),
                black_box(medium_new),
                &theme,
            ))
            .unwrap();
        })
    });

    group.bench_function("medium_signs", |b| {
        b.iter(|| {
            let mut buffer = Cursor::new(Vec::new());
            let theme = SignsTheme::default();
            black_box(diff(
                &mut buffer,
                black_box(medium_old),
                black_box(medium_new),
                &theme,
            ))
            .unwrap();
        })
    });

    // Large inputs
    group.bench_function("large_arrows", |b| {
        b.iter(|| {
            let mut buffer = Cursor::new(Vec::new());
            let theme = ArrowsTheme::default();
            black_box(diff(
                &mut buffer,
                black_box(large_old),
                black_box(large_new),
                &theme,
            ))
            .unwrap();
        })
    });

    group.bench_function("large_signs", |b| {
        b.iter(|| {
            let mut buffer = Cursor::new(Vec::new());
            let theme = SignsTheme::default();
            black_box(diff(
                &mut buffer,
                black_box(large_old),
                black_box(large_new),
                &theme,
            ))
            .unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_diff);
criterion_main!(benches);
