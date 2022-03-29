#[macro_export]
#[rustfmt::rust_code]
macro_rules! benchmarks {
    ($($(@ $commands:ident)* $name: ident $block: block)+) => {
        #[allow(unused_imports)]
        use criterion::{black_box, Criterion};
        fn command_used(commands: &str, command: &str) -> bool {
            commands.split(" ").collect::<Vec<&str>>().contains(&command)
        }
        pub fn criterion_benchmark(criterion: &mut Criterion) {
            $(
                let commands = stringify!($($commands)*);
                if !command_used(commands, "skip") {
                    criterion.bench_function(stringify!($name), |bencher| bencher.iter(||$block));
                }
            )+
        }
        criterion::criterion_group!(benches, criterion_benchmark);
        criterion::criterion_main!(benches);
    };
}
