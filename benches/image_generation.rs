use brutemoji::generate_image;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    use image::open;
    use std::path::Path;

    let img = open(Path::new("./assets/georgia.jpg")).unwrap();
    let path = Path::new("./bench.png");

    c.bench_function("image generation 30", |b| {
        b.iter(|| {
            generate_image(
                black_box(&img),
                black_box(30),
                black_box(true),
                black_box(path),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
