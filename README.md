# Benchmark Scratchpad

A quick scratchpad for benchmarking Rust code

## Prerequisites

- [cargo-make](https://github.com/sagiegurari/cargo-make) - `cargo install --force cargo-make`

## Instructions

1. Edit [benches/benchmark.rs](benches/benchmark.rs)
2. Run [cargo-make](https://github.com/sagiegurari/cargo-make) - `cargo make bench`

## `benchmarks!`

The `benchmarks!` macro accepts input with the following structure:

```
benchmarks! {
  benchmark_name { /* benchmark impl */ }
  other_benchmark_name { /* other benchmark impl */ }
}
```

The [criterion.rs](https://github.com/bheisler/criterion.rs) `black_box` function is included and usable within the macro.

`criterion::black_box` stops the compiler from constant-folding away the whole function and replacing it with a constant.

## Example

```rust 
mod internals;

benchmarks! {
    collect_vec_string {
        let vector = black_box(vec!["hello", "world"]);
        let _output = vector
            .iter()
            .map(|item| item.to_uppercase())
            .collect::<Vec<String>>()
            .join("");
    }

    collect_string {
        let vector = black_box(vec!["hello", "world"]);
        let _output = vector
            .iter()
            .map(|item| item.to_uppercase())
            .collect::<String>();
    }
}
```

## Tasks

- `cargo make bench` - runs the benchmark via `cargo-criterion`
- `cargo make reset` - resets the criterion data for earlier benchmarks
- `cargo make new` - returns [benches/benchmark.rs](benches/benchmark.rs) to the initial state

## Uses 

- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [cargo-criterion](https://github.com/bheisler/cargo-criterion)
- [criterion.rs](https://github.com/bheisler/criterion.rs)
