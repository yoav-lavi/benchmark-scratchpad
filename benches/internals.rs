#[macro_export]
macro_rules! benchmarks {
    ($($name: ident $block: block)+) => {
        #[allow(unused_imports)]
        use criterion::black_box;
        use criterion::Criterion;
        pub fn criterion_benchmark(criterion: &mut Criterion) {
            $(criterion.bench_function(stringify!($name), |bencher| bencher.iter(||$block));)+
        }
        criterion::criterion_group!(benches, criterion_benchmark);
        criterion::criterion_main!(benches);
    };
}
