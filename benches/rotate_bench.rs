#[macro_use]
extern crate criterion;

use criterion::Criterion;
//use criterion::black_box;
use bitarray::bitarray::BitArray;

fn criterion_benchmark(c: &mut Criterion) {
    let mut arr = BitArray::new(40);
    
    c.bench_function("BitArray::randfill", |b| b.iter(|| arr.randfill() ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
